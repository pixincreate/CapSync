use capsync::clone::parse_repo_url;

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
}

#[test]
fn test_parse_owner_repo_shorthand() {
    let result = parse_repo_url("pixincreate/capsync").unwrap();
    assert_eq!(result, "https://github.com/pixincreate/capsync.git");

    let result2 = parse_repo_url("pixincreate/capsync/").unwrap();
    assert_eq!(result2, "https://github.com/pixincreate/capsync.git");

    let result3 = parse_repo_url(" owner/repo ").unwrap();
    assert_eq!(result3, "https://github.com/owner/repo.git");
}
