pub use crate::git::{clone_to_path, get_remote_default_branch};

use crate::config::Config;
use crate::git::build_fetch_options;
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

pub fn normalize_repo_identity(input: &str) -> Option<String> {
    let normalized_input = input.trim().trim_end_matches('/').trim_end_matches(".git");

    if normalized_input.is_empty() {
        return None;
    }

    if !normalized_input.contains("://") && !normalized_input.starts_with("git@") {
        return normalize_repo_identity_parts("github.com", normalized_input);
    }

    if let Some(ssh_target) = normalized_input.strip_prefix("git@") {
        let (host, path) = ssh_target.split_once(':')?;
        return normalize_repo_identity_parts(host, path);
    }

    let (_, remainder) = normalized_input.split_once("://")?;
    let (authority, path) = remainder.split_once('/')?;
    let host = authority.rsplit('@').next()?.split(':').next()?;

    normalize_repo_identity_parts(host, path)
}

fn normalize_repo_identity_parts(host: &str, path: &str) -> Option<String> {
    let normalized_path = path.trim_start_matches('/');
    let mut segments = normalized_path.split('/');
    let owner = segments.next()?;
    let repository_name = segments.next()?;

    if segments.next().is_some()
        || host.is_empty()
        || owner.is_empty()
        || repository_name.is_empty()
    {
        return None;
    }

    Some(format!(
        "{}/{}/{}",
        host.to_lowercase(),
        owner,
        repository_name
    ))
}

pub fn parse_repo_url(input: &str) -> Result<String> {
    let input = input.trim();

    if !input.contains("://") && !input.starts_with("git@") {
        let shorthand = input.trim_end_matches('/').trim_end_matches(".git");
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

pub fn update_existing(path: &Path) -> Result<()> {
    let repository = Repository::open(path).context("Failed to open existing repository")?;

    if has_unpushed_changes(path) {
        return Err(anyhow!(
            "Cannot update: working tree has uncommitted or unpushed changes. Commit or stash them first."
        ));
    }

    let mut fetch_options = build_fetch_options();

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
                let is_same_repo = match (
                    normalize_repo_identity(remote_url),
                    normalize_repo_identity(&url),
                ) {
                    (Some(current_repo), Some(requested_repo)) => current_repo == requested_repo,
                    _ => remote_url == &url,
                };

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
                                println!(
                                    "\nRequested branch '{}' differs from current local branch '{}'.",
                                    requested_branch, current_branch
                                );

                                loop {
                                    print!(
                                        "Re-clone the requested branch instead of updating in place? [y/N]: "
                                    );
                                    io::stdout().flush()?;
                                    let mut branch_input = String::new();
                                    io::stdin().read_line(&mut branch_input)?;
                                    let branch_input = branch_input.trim().to_lowercase();

                                    if branch_input == "y" {
                                        break;
                                    } else if branch_input.is_empty() || branch_input == "n" {
                                        return Err(anyhow!("Aborted."));
                                    }
                                    println!("Please enter y or n.");
                                }

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
