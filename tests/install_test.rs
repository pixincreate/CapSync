use capsync::config::Config;
use capsync::install::{
    InstallOptions, InstallResult, ResolvedInstallRef, SkillSelector, install_skill,
    install_skill_from_checkout, normalize_skill_slug, resolve_install_ref,
};
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

fn write_skill(skill_dir: &std::path::Path, name: &str, description: &str) {
    fs::create_dir_all(skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        format!(
            "---\nname: \"{}\"\ndescription: \"{}\"\n---\n\n# {}\n",
            name, description, name
        ),
    )
    .unwrap();
}

#[test]
fn test_resolve_install_ref_skills_sh_url() {
    let resolved = resolve_install_ref("https://skills.sh/vercel-labs/skills/find-skills").unwrap();

    assert_eq!(
        resolved.repo_url,
        "https://github.com/vercel-labs/skills.git"
    );
    assert_eq!(resolved.branch, None);
    assert_eq!(
        resolved.selector,
        SkillSelector::Slug("find-skills".to_string())
    );
}

#[test]
fn test_resolve_install_ref_http_skills_sh_url() {
    let resolved = resolve_install_ref("http://skills.sh/vercel-labs/skills/find-skills").unwrap();

    assert_eq!(
        resolved.repo_url,
        "https://github.com/vercel-labs/skills.git"
    );
    assert_eq!(resolved.branch, None);
    assert_eq!(
        resolved.selector,
        SkillSelector::Slug("find-skills".to_string())
    );
}

#[test]
fn test_resolve_install_ref_github_tree_url() {
    let resolved =
        resolve_install_ref("https://github.com/vercel-labs/skills/tree/main/skills/find-skills")
            .unwrap();

    assert_eq!(
        resolved.repo_url,
        "https://github.com/vercel-labs/skills.git"
    );
    assert_eq!(resolved.branch, Some("main".to_string()));
    assert_eq!(
        resolved.selector,
        SkillSelector::Path(PathBuf::from("skills/find-skills"))
    );
}

#[test]
fn test_resolve_install_ref_owner_repo_slug() {
    let resolved = resolve_install_ref("vercel-labs/skills/find-skills").unwrap();

    assert_eq!(
        resolved.repo_url,
        "https://github.com/vercel-labs/skills.git"
    );
    assert_eq!(resolved.branch, None);
    assert_eq!(
        resolved.selector,
        SkillSelector::Slug("find-skills".to_string())
    );
}

#[test]
fn test_resolve_install_ref_owner_repo_path() {
    let resolved = resolve_install_ref("vercel-labs/skills/skills/find-skills").unwrap();

    assert_eq!(
        resolved.repo_url,
        "https://github.com/vercel-labs/skills.git"
    );
    assert_eq!(resolved.branch, None);
    assert_eq!(
        resolved.selector,
        SkillSelector::Path(PathBuf::from("skills/find-skills"))
    );
}

#[test]
fn test_resolve_install_ref_rejects_repo_only_reference() {
    let error = resolve_install_ref("vercel-labs/skills").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Install requires a concrete skill reference")
    );
}

#[test]
fn test_normalize_skill_slug() {
    assert_eq!(normalize_skill_slug("Find Skills"), "find-skills");
    assert_eq!(normalize_skill_slug("frontend_design"), "frontend-design");
    assert_eq!(normalize_skill_slug("  !!!  "), "");
}

#[test]
fn test_install_skill_from_checkout_by_slug_uses_skill_name_slug() {
    let checkout_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();

    let skill_dir = checkout_dir.path().join("skills").join("find-skills");
    write_skill(&skill_dir, "Find Skills", "Locate useful skills");
    fs::write(skill_dir.join("notes.txt"), "extra file").unwrap();

    let resolved = ResolvedInstallRef {
        repo_url: "https://github.com/vercel-labs/skills.git".to_string(),
        branch: None,
        selector: SkillSelector::Slug("find-skills".to_string()),
    };

    let result =
        install_skill_from_checkout(checkout_dir.path(), &resolved, target_dir.path()).unwrap();

    assert_eq!(result.skill_slug, "find-skills");
    assert!(!result.replaced_existing);
    assert!(result.installed_path.join("SKILL.md").exists());
    assert!(result.installed_path.join("notes.txt").exists());
}

#[test]
fn test_install_skill_from_checkout_prefers_skill_name_for_slug() {
    let checkout_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();

    let skill_dir = checkout_dir.path().join("skills").join("find-skill-files");
    write_skill(&skill_dir, "Find Skills", "Locate useful skills");

    let resolved = ResolvedInstallRef {
        repo_url: "https://github.com/vercel-labs/skills.git".to_string(),
        branch: None,
        selector: SkillSelector::Path(PathBuf::from("skills/find-skill-files")),
    };

    let result =
        install_skill_from_checkout(checkout_dir.path(), &resolved, target_dir.path()).unwrap();

    assert_eq!(result.skill_slug, "find-skills");
    assert_eq!(result.installed_path, target_dir.path().join("find-skills"));
}

#[test]
fn test_install_skill_from_checkout_by_explicit_path() {
    let checkout_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();

    let skill_dir = checkout_dir.path().join("packages").join("frontend-design");
    write_skill(&skill_dir, "Frontend Design", "Build polished interfaces");

    let resolved = ResolvedInstallRef {
        repo_url: "https://github.com/vercel-labs/skills.git".to_string(),
        branch: Some("main".to_string()),
        selector: SkillSelector::Path(PathBuf::from("packages/frontend-design")),
    };

    let result =
        install_skill_from_checkout(checkout_dir.path(), &resolved, target_dir.path()).unwrap();

    assert_eq!(result.skill_slug, "frontend-design");
    assert!(result.installed_path.join("SKILL.md").exists());
}

#[test]
fn test_install_skill_from_checkout_errors_on_missing_skill() {
    let checkout_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();

    let resolved = ResolvedInstallRef {
        repo_url: "https://github.com/vercel-labs/skills.git".to_string(),
        branch: None,
        selector: SkillSelector::Slug("missing-skill".to_string()),
    };

    let error =
        install_skill_from_checkout(checkout_dir.path(), &resolved, target_dir.path()).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Could not find a skill matching")
    );
}

#[test]
fn test_install_skill_from_checkout_errors_on_ambiguous_slug() {
    let checkout_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();

    write_skill(
        &checkout_dir.path().join("skills").join("frontend-design"),
        "Frontend Design",
        "Primary skill",
    );
    write_skill(
        &checkout_dir.path().join("alt").join("frontend-design-copy"),
        "Frontend Design",
        "Duplicate by name",
    );

    let resolved = ResolvedInstallRef {
        repo_url: "https://github.com/vercel-labs/skills.git".to_string(),
        branch: None,
        selector: SkillSelector::Slug("frontend-design".to_string()),
    };

    let error =
        install_skill_from_checkout(checkout_dir.path(), &resolved, target_dir.path()).unwrap_err();
    assert!(error.to_string().contains("Found multiple skills matching"));
}

#[test]
fn test_install_result_reports_installed_path() {
    let checkout_dir = tempdir().unwrap();
    let target_dir = tempdir().unwrap();

    let skill_dir = checkout_dir.path().join("skills").join("find-skills");
    write_skill(&skill_dir, "Find Skills", "Locate useful skills");

    let options = InstallOptions {
        reference: "vercel-labs/skills/find-skills".to_string(),
    };
    let resolved = ResolvedInstallRef {
        repo_url: "https://github.com/vercel-labs/skills.git".to_string(),
        branch: None,
        selector: SkillSelector::Slug("find-skills".to_string()),
    };

    let result: InstallResult =
        install_skill_from_checkout(checkout_dir.path(), &resolved, target_dir.path()).unwrap();

    assert_eq!(options.reference, "vercel-labs/skills/find-skills");
    assert_eq!(result.skill_slug, "find-skills");
    assert_eq!(result.installed_path, target_dir.path().join("find-skills"));
    assert!(!result.replaced_existing);
}

#[test]
fn test_install_skill_rejects_git_repo_skills_source() {
    let repository_dir = tempdir().unwrap();
    git2::Repository::init(repository_dir.path()).unwrap();

    let config = Config {
        skills_source: repository_dir.path().to_path_buf(),
        commands_source: None,
        destinations: Config::default().destinations,
    };

    let options = InstallOptions {
        reference: "vercel-labs/skills/find-skills".to_string(),
    };

    let error = install_skill(&options, &config).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Skills source is currently a git repository")
    );
}
