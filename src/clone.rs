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

    if input.starts_with("http://") || input.starts_with("https://") || input.starts_with("git@") {
        if input.ends_with(".git") {
            Ok(input.to_string())
        } else {
            Ok(format!("{}.git", input))
        }
    } else if input.contains('/') && !input.contains(':') {
        Ok(format!("https://github.com/{}.git", input))
    } else {
        Err(anyhow!(
            "Invalid repository: '{}'. Use 'owner/repo' or full URL.",
            input
        ))
    }
}

pub fn get_remote_default_branch(url: &str) -> Result<String> {
    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let repository =
        Repository::init(temp_dir.path()).context("Failed to initialize temp repository")?;

    let mut remote = repository
        .remote_anonymous(url)
        .context("Failed to create remote")?;

    remote
        .fetch(
            &["+refs/heads/*:refs/remotes/origin/*"],
            Some(&mut fetch_options),
            None,
        )
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

    if let Ok(status) = repository.statuses(None) {
        return status.iter().any(|entry| {
            let status_flag = entry.status();
            status_flag != git2::Status::INDEX_NEW
                && status_flag != git2::Status::INDEX_MODIFIED
                && status_flag != git2::Status::INDEX_DELETED
                && status_flag != git2::Status::INDEX_RENAMED
                && status_flag != git2::Status::INDEX_TYPECHANGE
                && status_flag != git2::Status::WT_NEW
                && status_flag != git2::Status::WT_MODIFIED
                && status_flag != git2::Status::WT_DELETED
                && status_flag != git2::Status::WT_RENAMED
                && status_flag != git2::Status::WT_TYPECHANGE
                && status_flag != git2::Status::IGNORED
        });
    }

    false
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

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);

    let remotes = repository.remotes().context("Failed to get remotes")?;

    for remote_name in remotes.iter().flatten() {
        if let Ok(mut remote) = repository.find_remote(remote_name) {
            let _ = remote.fetch(
                &["+refs/heads/*:refs/remotes/origin/*"],
                Some(&mut fetch_options),
                None,
            );
        }
    }

    let head = repository.head().context("Failed to get HEAD")?;

    let branch_name = head
        .shorthand()
        .ok_or_else(|| anyhow!("Cannot determine branch name"))?;

    let remote_ref = format!("origin/{}", branch_name);

    if let Ok(reference) = repository.find_reference(&remote_ref) {
        if let Ok(commit) = reference.peel_to_commit() {
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

    Ok(())
}

pub fn get_remote_url(path: &Path) -> Option<String> {
    let repository = Repository::open(path).ok()?;
    let remote = repository.find_remote("origin").ok()?;
    remote.url().map(|url_string| url_string.to_string())
}

pub fn clone_skills(options: &CloneOptions, config: &Config) -> Result<CloneResult> {
    let url = parse_repo_url(&options.repo)?;

    let source = &config.skills_source;
    let source_exists = source.exists();

    let branch = if let Some(branch_name) = &options.branch {
        branch_name.clone()
    } else {
        println!("Fetching remote branch info...");
        get_remote_default_branch(&url)?
    };

    println!("Using branch: {}", branch);

    let (action, backup_path) = if source_exists {
        let current_remote = get_remote_url(source);

        if current_remote.is_some() {
            println!("\nSkills source already exists.");

            let is_same_repo = current_remote
                .as_ref()
                .map(|remote_url| remote_url.contains(&options.repo) || url.contains(remote_url))
                .unwrap_or(false);

            if is_same_repo {
                loop {
                    print!("Update (git pull) or Override (backup + fresh clone)? [U/o]: ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim().to_lowercase();

                    if input.is_empty() || input == "u" {
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
    clone_to_path(&url, &branch, source)?;

    println!("Successfully cloned {} (branch: {})", options.repo, branch);

    Ok(CloneResult {
        action,
        backup_path,
    })
}
