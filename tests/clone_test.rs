use capsync::clone::{
    get_remote_default_branch, get_remote_url, normalize_repo_identity, parse_repo_url,
};
use git2::{Repository, Signature};
use std::fs;
use tempfile::tempdir;

fn create_commit(repository: &Repository) -> git2::Oid {
    let signature = Signature::now("CapSync Tests", "tests@capsync.dev").unwrap();
    let tree_id = {
        let mut index = repository.index().unwrap();
        let workdir = repository.workdir().unwrap();
        let file_path = workdir.join("SKILL.md");
        fs::write(&file_path, "# test\n").unwrap();
        index.add_path(std::path::Path::new("SKILL.md")).unwrap();
        index.write_tree().unwrap()
    };

    let tree = repository.find_tree(tree_id).unwrap();
    repository
        .commit(Some("HEAD"), &signature, &signature, "initial", &tree, &[])
        .unwrap()
}

#[test]
fn test_parse_full_https_url() {
    let result = parse_repo_url("https://github.com/user/repo").unwrap();
    assert_eq!(result, "https://github.com/user/repo.git");

    let result2 = parse_repo_url("https://github.com/user/repo.git").unwrap();
    assert_eq!(result2, "https://github.com/user/repo.git");
}

#[test]
fn test_parse_full_http_url() {
    let result = parse_repo_url("http://github.com/user/repo").unwrap();
    assert_eq!(result, "http://github.com/user/repo.git");
}

#[test]
fn test_parse_git_ssh_url() {
    let result = parse_repo_url("git@github.com:user/repo").unwrap();
    assert_eq!(result, "git@github.com:user/repo.git");

    let result2 = parse_repo_url("git@github.com:user/repo.git").unwrap();
    assert_eq!(result2, "git@github.com:user/repo.git");
}

#[test]
fn test_parse_non_github_url() {
    let result = parse_repo_url("https://codeberg.org/user/repo").unwrap();
    assert_eq!(result, "https://codeberg.org/user/repo.git");

    let result2 = parse_repo_url("https://gitlab.com/user/repo.git").unwrap();
    assert_eq!(result2, "https://gitlab.com/user/repo.git");

    let result3 = parse_repo_url("git@bitbucket.org:user/repo.git").unwrap();
    assert_eq!(result3, "git@bitbucket.org:user/repo.git");
}

#[test]
fn test_parse_url_trims_whitespace() {
    let result = parse_repo_url("  https://github.com/user/repo  ").unwrap();
    assert_eq!(result, "https://github.com/user/repo.git");
}

#[test]
fn test_parse_url_trims_trailing_slash() {
    let result = parse_repo_url("https://github.com/user/repo/").unwrap();
    assert_eq!(result, "https://github.com/user/repo.git");
}

#[test]
fn test_parse_invalid_url() {
    let result = parse_repo_url("not-a-url");
    assert!(result.is_err());

    let result2 = parse_repo_url("");
    assert!(result2.is_err());

    let result3 = parse_repo_url("owner/repo/extra");
    assert!(result3.is_err());
}

#[test]
fn test_parse_owner_repo_shorthand() {
    let result = parse_repo_url("pixincreate/capsync").unwrap();
    assert_eq!(result, "https://github.com/pixincreate/capsync.git");

    let result2 = parse_repo_url("pixincreate/capsync/").unwrap();
    assert_eq!(result2, "https://github.com/pixincreate/capsync.git");

    let result3 = parse_repo_url(" owner/repo ").unwrap();
    assert_eq!(result3, "https://github.com/owner/repo.git");

    let result4 = parse_repo_url("owner/repo.git").unwrap();
    assert_eq!(result4, "https://github.com/owner/repo.git");
}

#[test]
fn test_normalize_repo_identity_matches_same_repo_across_url_formats() {
    let https_identity = normalize_repo_identity("https://github.com/pixincreate/CapSync.git");
    let ssh_identity = normalize_repo_identity("git@github.com:pixincreate/CapSync.git");
    let shorthand_identity = normalize_repo_identity("pixincreate/CapSync");
    let ssh_url_identity = normalize_repo_identity("ssh://git@github.com/pixincreate/CapSync");

    assert_eq!(
        https_identity,
        Some("github.com/pixincreate/CapSync".to_string())
    );
    assert_eq!(https_identity, ssh_identity);
    assert_eq!(https_identity, shorthand_identity);
    assert_eq!(https_identity, ssh_url_identity);
}

#[test]
fn test_normalize_repo_identity_rejects_invalid_paths() {
    assert_eq!(normalize_repo_identity(""), None);
    assert_eq!(normalize_repo_identity("owner/repo/extra"), None);
    assert_eq!(normalize_repo_identity("https://github.com/owner"), None);
}

#[test]
fn test_get_remote_url_distinguishes_missing_origin_from_non_repo() {
    let temp_dir = tempdir().unwrap();
    let repository = Repository::init(temp_dir.path()).unwrap();
    create_commit(&repository);

    let remote_url = get_remote_url(temp_dir.path()).unwrap();
    assert_eq!(remote_url, None);

    let non_repo_dir = tempdir().unwrap();
    let error = get_remote_url(non_repo_dir.path()).unwrap_err();
    assert!(error.to_string().contains("Failed to open repository"));
}

#[test]
fn test_get_remote_default_branch_uses_remote_head() {
    let temp_dir = tempdir().unwrap();
    let repository = Repository::init(temp_dir.path()).unwrap();
    let commit_id = create_commit(&repository);

    let commit = repository.find_commit(commit_id).unwrap();
    repository.branch("develop", &commit, true).unwrap();
    repository.branch("main", &commit, true).unwrap();
    repository.set_head("refs/heads/develop").unwrap();

    let default_branch = get_remote_default_branch(temp_dir.path().to_str().unwrap()).unwrap();
    assert_eq!(default_branch, "develop");
}
