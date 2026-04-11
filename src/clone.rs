use crate::config::Config;
use anyhow::{Context, Result, anyhow};
use git2::Repository;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct CloneResult {
    pub action: CloneAction,
    pub backup_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CloneAction {
    Cloned,
    Updated,
    Overridden,
}

pub struct CloneOptions {
    pub repo: String,
    pub branch: Option<String>,
}

pub fn parse_repo_url(input: &str) -> Result<String> {
    let input = input.trim();

    if !input.contains("://") && !input.starts_with("git@") {
        let shorthand = input.trim_end_matches('/');
        let parts: Vec<&str> = shorthand.split('/').collect();
        if parts.len() == 2 && parts.iter().all(|part| !part.is_empty()) {
            return Ok(format!("https://github.com/{}.git", shorthand));
        }
    }

    if input.starts_with("http://") || input.starts_with("https://") || input.starts_with("git@") {
        let normalized = input.trim_end_matches('/');
        if normalized.ends_with(".git") {
            Ok(normalized.to_string())
        } else {
            Ok(format!("{}.git", normalized))
        }
    } else {
        Err(anyhow!(
            "Invalid repository: '{}'. Must be a valid git URL or owner/repo.",
            input
        ))
    }
}

pub fn get_remote_default_branch(url: &str) -> Result<String> {
    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

    let repository =
        Repository::init(temp_dir.path()).context("Failed to initialize temp repository")?;

    let mut remote = repository
        .remote_anonymous(url)
        .context("Failed to create remote")?;

    remote
        .connect(git2::Direction::Fetch)
        .context("Failed to connect to remote")?;

    if let Ok(default_branch) = remote.default_branch() {
        remote.disconnect().context("Failed to disconnect remote")?;

        let branch_name = default_branch
            .as_str()
            .context("Failed to read default branch name")?
            .trim_start_matches("refs/heads/")
            .to_string();

        if !branch_name.is_empty() {
            return Ok(branch_name);
        }
    } else {
        remote.disconnect().context("Failed to disconnect remote")?;
    }

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let remote_name = remote.name().unwrap_or("origin");
    let refspec = format!("+refs/heads/*:refs/remotes/{}/*", remote_name);

    remote
        .fetch(&[&refspec], Some(&mut fetch_options), None)
        .context("Failed to fetch from remote")?;

    let default_branch = find_default_branch(&repository)?;

    Ok(default_branch)
}

fn find_default_branch(repository: &Repository) -> Result<String> {
    let branches = repository.branches(Some(git2::BranchType::Remote))?;

    let mut candidates: Vec<(String, bool)> = Vec::new();

    for branch_result in branches {
        let (branch, _) = branch_result.context("Failed to iterate branches")?;
        let name_result = branch.name().context("Failed to get branch name")?;
        if let Some(branch_name) = name_result {
            if branch_name.ends_with("/HEAD") {
                continue;
            }
            let is_main = branch_name == "origin/main";
            let is_master = branch_name == "origin/master";
            let is_preferred = is_main || is_master;
            candidates.push((branch_name.to_string(), is_preferred));
        }
    }

    candidates.sort_by(|candidate_a, candidate_b| {
        let (_, a_is_preferred) = candidate_a;
        let (_, b_is_preferred) = candidate_b;
        match (a_is_preferred, b_is_preferred) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => candidate_a.0.cmp(&candidate_b.0),
        }
    });

    candidates
        .first()
        .map(|(branch_name, _)| branch_name.trim_start_matches("origin/").to_string())
        .ok_or_else(|| anyhow!("No branches found in remote repository"))
}

pub fn has_unpushed_changes(path: &Path) -> bool {
    let repository = match Repository::open(path) {
        Ok(repo) => repo,
        Err(_) => return false,
    };

    let mut status_options = git2::StatusOptions::new();
    status_options
        .include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false);

    if let Ok(statuses) = repository.statuses(Some(&mut status_options)) {
        let has_uncommitted = statuses.iter().any(|entry| {
            let status = entry.status();
            status.is_index_new()
                || status.is_index_modified()
                || status.is_index_deleted()
                || status.is_index_renamed()
                || status.is_index_typechange()
                || status.is_wt_new()
                || status.is_wt_modified()
                || status.is_wt_deleted()
                || status.is_wt_renamed()
                || status.is_wt_typechange()
                || status.is_conflicted()
        });

        if has_uncommitted {
            return true;
        }
    }

    let head = match repository.head() {
        Ok(h) if h.is_branch() => h,
        _ => return false,
    };

    let local_oid = match head.target() {
        Some(oid) => oid,
        None => return false,
    };

    let branch_name = match head.shorthand() {
        Some(name) => name,
        None => return false,
    };

    let local_branch = match repository.find_branch(branch_name, git2::BranchType::Local) {
        Ok(branch) => branch,
        Err(_) => return false,
    };

    let upstream = match local_branch.upstream() {
        Ok(branch) => branch,
        Err(_) => return false,
    };

    let upstream_oid = match upstream.get().target() {
        Some(oid) => oid,
        None => return false,
    };

    match repository.graph_ahead_behind(local_oid, upstream_oid) {
        Ok((ahead, _)) => ahead > 0,
        Err(_) => false,
    }
}

pub fn backup_existing(source: &Path) -> Result<PathBuf> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .context("Failed to get timestamp")?
        .as_secs();

    let backup_name = format!("skills_source.backup.{}", timestamp);
    let backup_path = source
        .parent()
        .ok_or_else(|| anyhow!("Cannot determine backup location"))?
        .join(&backup_name);

    std::fs::rename(source, &backup_path).context("Failed to move existing skills to backup")?;

    Ok(backup_path)
}

pub fn clone_to_path(url: &str, branch: &str, target: &Path) -> Result<()> {
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    if !branch.is_empty() {
        builder.branch(branch);
    }

    builder
        .clone(url, target)
        .context(format!("Failed to clone from {}", url))?;

    Ok(())
}

pub fn update_existing(path: &Path) -> Result<()> {
    let repository = Repository::open(path).context("Failed to open existing repository")?;

    if has_unpushed_changes(path) {
        return Err(anyhow!(
            "Cannot update: working tree has uncommitted or unpushed changes. Commit or stash them first."
        ));
    }

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let remotes = repository.remotes().context("Failed to get remotes")?;

    for remote_name in remotes.iter().flatten() {
        let mut remote = repository
            .find_remote(remote_name)
            .with_context(|| format!("Failed to find remote '{}'", remote_name))?;

        let refspec = format!("+refs/heads/*:refs/remotes/{}/*", remote_name);
        remote
            .fetch(&[&refspec], Some(&mut fetch_options), None)
            .with_context(|| format!("Failed to fetch from remote '{}'", remote_name))?;
    }

    let head = repository.head().context("Failed to get HEAD")?;

    let branch_name = head
        .shorthand()
        .ok_or_else(|| anyhow!("Cannot determine branch name"))?;

    if let Ok(local_branch) = repository.find_branch(branch_name, git2::BranchType::Local) {
        if let Ok(upstream_branch) = local_branch.upstream() {
            if let Ok(commit) = upstream_branch.get().peel_to_commit() {
                let mut checkout_options = git2::build::CheckoutBuilder::new();
                checkout_options.force();
                repository
                    .reset(
                        &commit.into_object(),
                        git2::ResetType::Hard,
                        Some(&mut checkout_options),
                    )
                    .context("Failed to reset to latest")?;
            }
        }
    }

    Ok(())
}

pub fn get_remote_url(path: &Path) -> Result<Option<String>> {
    let repository = Repository::open(path).context("Failed to open repository")?;
    match repository.find_remote("origin") {
        Ok(remote) => Ok(remote.url().map(|url_string| url_string.to_string())),
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
        Err(e) => Err(e).context("Failed to find origin remote"),
    }
}

fn open_repository(path: &Path) -> Result<Option<Repository>> {
    match Repository::open(path) {
        Ok(repository) => Ok(Some(repository)),
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
        Err(e) => Err(e).context("Failed to open repository"),
    }
}

fn current_branch_name(repository: &Repository) -> Result<String> {
    repository
        .head()
        .context("Failed to get HEAD")?
        .shorthand()
        .ok_or_else(|| anyhow!("Cannot determine branch name"))
        .map(str::to_string)
}

pub fn clone_skills(options: &CloneOptions, config: &Config) -> Result<CloneResult> {
    let url = parse_repo_url(&options.repo)?;

    let source = &config.skills_source;
    let source_exists = source.exists();

    let requested_branch = if let Some(branch_name) = &options.branch {
        branch_name.clone()
    } else {
        println!("Fetching remote branch info...");
        get_remote_default_branch(&url)?
    };

    println!("Using branch: {}", requested_branch);

    let (action, backup_path) = if source_exists {
        let repository = open_repository(source)?;

        if repository.is_some() {
            let current_remote = get_remote_url(source)?;
            println!("\nSkills source already exists.");

            if let Some(remote_url) = current_remote.as_ref() {
                let is_same_repo = remote_url.contains(&options.repo) || url.contains(remote_url);

                if is_same_repo {
                    loop {
                        print!("Update (git pull) or Override (download new)? [U/o]: ");
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        let input = input.trim().to_lowercase();

                        if input.is_empty() || input == "u" {
                            let existing_repository = Repository::open(source)
                                .context("Failed to open existing repository")?;
                            let current_branch = current_branch_name(&existing_repository)?;

                            if current_branch != requested_branch {
                                break;
                            }

                            update_existing(source)?;
                            return Ok(CloneResult {
                                action: CloneAction::Updated,
                                backup_path: None,
                            });
                        } else if input == "o" {
                            break;
                        }
                        println!("Please enter U or o.");
                    }
                } else {
                    loop {
                        print!("Override with different repository? [y/N]: ");
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        let input = input.trim().to_lowercase();

                        if input == "y" {
                            break;
                        } else if input.is_empty() || input == "n" {
                            return Err(anyhow!("Aborted."));
                        }
                        println!("Please enter y or n.");
                    }
                }
            } else {
                loop {
                    print!("Skills source exists but has no origin remote. Override? [y/N]: ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim().to_lowercase();

                    if input == "y" {
                        break;
                    } else if input.is_empty() || input == "n" {
                        return Err(anyhow!("Aborted."));
                    }
                    println!("Please enter y or n.");
                }
            }
        } else {
            loop {
                print!("Skills source exists but is not a git repository. Override? [y/N]: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim().to_lowercase();

                if input == "y" {
                    break;
                } else if input.is_empty() || input == "n" {
                    return Err(anyhow!("Aborted."));
                }
                println!("Please enter y or n.");
            }
        }

        if has_unpushed_changes(source) {
            println!("\nWARNING: Local skills_source has uncommitted or unpushed changes.");
            loop {
                print!("Backup and override? [y/N]: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim().to_lowercase();

                if input == "y" {
                    let backup = backup_existing(source)?;
                    break (CloneAction::Overridden, Some(backup));
                } else if input.is_empty() || input == "n" {
                    return Err(anyhow!("Aborted."));
                }
                println!("Please enter y or n.");
            }
        } else {
            std::fs::remove_dir_all(source).context("Failed to remove existing skills_source")?;
            (CloneAction::Overridden, None)
        }
    } else {
        if let Some(parent) = source.parent() {
            std::fs::create_dir_all(parent)?;
        }
        (CloneAction::Cloned, None)
    };

    println!("Cloning into {}...", source.display());
    clone_to_path(&url, &requested_branch, source)?;

    println!(
        "Successfully cloned {} (branch: {})",
        options.repo, requested_branch
    );

    Ok(CloneResult {
        action,
        backup_path,
    })
}
