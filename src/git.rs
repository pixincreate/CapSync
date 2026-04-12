use anyhow::{Context, Result, anyhow};
use git2::Repository;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
enum AuthAction {
    Username(String),
    SshAgent(String),
    CredentialHelper,
    Default,
    Unsupported,
}

pub fn get_remote_default_branch(url: &str) -> Result<String> {
    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

    let repository =
        Repository::init(temp_dir.path()).context("Failed to initialize temp repository")?;

    let mut remote = repository
        .remote_anonymous(url)
        .context("Failed to create remote")?;

    let default_branch = {
        let connection = remote
            .connect_auth(git2::Direction::Fetch, Some(build_remote_callbacks()), None)
            .context("Failed to connect to remote")?;

        connection.default_branch().ok()
    };

    if let Some(default_branch) = default_branch {
        let branch_name = default_branch
            .as_str()
            .context("Failed to read default branch name")?
            .trim_start_matches("refs/heads/")
            .to_string();

        if !branch_name.is_empty() {
            return Ok(branch_name);
        }
    }

    let mut fetch_options = build_fetch_options();

    let remote_name = remote.name().unwrap_or("origin");
    let refspec = format!("+refs/heads/*:refs/remotes/{}/*", remote_name);

    remote
        .fetch(&[&refspec], Some(&mut fetch_options), None)
        .context("Failed to fetch from remote")?;

    find_default_branch(&repository)
}

pub fn clone_to_path(url: &str, branch: &str, target: &Path) -> Result<()> {
    let fetch_options = build_fetch_options();

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

pub(crate) fn build_fetch_options() -> git2::FetchOptions<'static> {
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);
    fetch_options.remote_callbacks(build_remote_callbacks());
    fetch_options
}

fn build_remote_callbacks() -> git2::RemoteCallbacks<'static> {
    let git_config = git2::Config::open_default().ok();
    let mut callbacks = git2::RemoteCallbacks::new();

    callbacks.credentials(
        move |url, username_from_url, allowed_types| match determine_auth_action(
            url,
            username_from_url,
            allowed_types,
        ) {
            AuthAction::Username(username) => git2::Cred::username(&username),
            AuthAction::SshAgent(username) => git2::Cred::ssh_key_from_agent(&username),
            AuthAction::CredentialHelper => {
                if let Some(config) = git_config.as_ref() {
                    if let Ok(credential) =
                        git2::Cred::credential_helper(config, url, username_from_url)
                    {
                        return Ok(credential);
                    }
                }

                if allowed_types.is_default() {
                    return git2::Cred::default();
                }

                Err(git2::Error::from_str(&format!(
                    "No credential helper credentials available for {}",
                    url
                )))
            }
            AuthAction::Default => git2::Cred::default(),
            AuthAction::Unsupported => Err(git2::Error::from_str(&format!(
                "No supported authentication method available for {}",
                url
            ))),
        },
    );

    callbacks
}

fn determine_auth_action(
    url: &str,
    username_from_url: Option<&str>,
    allowed_types: git2::CredentialType,
) -> AuthAction {
    if allowed_types.is_username() {
        if let Some(username) = infer_ssh_username(url, username_from_url) {
            return AuthAction::Username(username);
        }
    }

    if allowed_types.is_ssh_key() {
        if let Some(username) = infer_ssh_username(url, username_from_url) {
            return AuthAction::SshAgent(username);
        }
    }

    if allowed_types.is_user_pass_plaintext() {
        return AuthAction::CredentialHelper;
    }

    if allowed_types.is_default() {
        return AuthAction::Default;
    }

    AuthAction::Unsupported
}

fn infer_ssh_username(url: &str, username_from_url: Option<&str>) -> Option<String> {
    if let Some(username) = username_from_url {
        return Some(username.to_string());
    }

    if url.starts_with("git@") || url.starts_with("ssh://") {
        return Some("git".to_string());
    }

    None
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

#[cfg(test)]
mod tests {
    use super::{AuthAction, determine_auth_action};

    #[test]
    fn test_determine_auth_action_uses_username_for_ssh_urls() {
        let action = determine_auth_action(
            "git@github.com:remotion-dev/skills.git",
            None,
            git2::CredentialType::USERNAME,
        );

        assert_eq!(action, AuthAction::Username("git".to_string()));
    }

    #[test]
    fn test_determine_auth_action_uses_agent_for_ssh_keys() {
        let action = determine_auth_action(
            "ssh://git@github.com/remotion-dev/skills.git",
            Some("git"),
            git2::CredentialType::SSH_KEY,
        );

        assert_eq!(action, AuthAction::SshAgent("git".to_string()));
    }

    #[test]
    fn test_determine_auth_action_uses_helper_for_https_userpass() {
        let action = determine_auth_action(
            "https://github.com/remotion-dev/skills.git",
            None,
            git2::CredentialType::USER_PASS_PLAINTEXT,
        );

        assert_eq!(action, AuthAction::CredentialHelper);
    }

    #[test]
    fn test_determine_auth_action_uses_default_when_requested() {
        let action = determine_auth_action(
            "https://github.com/remotion-dev/skills.git",
            None,
            git2::CredentialType::DEFAULT,
        );

        assert_eq!(action, AuthAction::Default);
    }
}
