use crate::clone::clone_to_path;
use crate::config::Config;
use anyhow::{Context, Result, anyhow};
use git2::Repository;
use std::fs;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallOptions {
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallResult {
    pub skill_slug: String,
    pub installed_path: PathBuf,
    pub replaced_existing: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillSelector {
    Slug(String),
    Path(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedInstallRef {
    pub repo_url: String,
    pub branch: Option<String>,
    pub selector: SkillSelector,
}

pub fn resolve_install_ref(input: &str) -> Result<ResolvedInstallRef> {
    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        return Err(anyhow!(
            "Install reference cannot be empty. Use an HTTPS skills.sh URL, GitHub tree URL, or owner/repo/skill reference."
        ));
    }

    if trimmed_input.starts_with("http://skills.sh/") {
        return Err(anyhow!(
            "HTTP skills.sh references are not supported. Use https://skills.sh/owner/repo/skill-slug"
        ));
    }

    if let Some(path) = trimmed_input.strip_prefix("https://skills.sh/") {
        let parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
        if parts.len() == 3 {
            let owner = parts[0];
            let repository_name = parts[1];
            let skill_slug = parts[2];
            return Ok(ResolvedInstallRef {
                repo_url: format!("https://github.com/{owner}/{repository_name}.git"),
                branch: None,
                selector: SkillSelector::Slug(skill_slug.to_string()),
            });
        }

        return Err(anyhow!(
            "skills.sh references must point to a concrete skill, like https://skills.sh/owner/repo/skill-slug"
        ));
    }

    if let Some(tree_ref) = resolve_github_tree_ref(trimmed_input)? {
        return Ok(tree_ref);
    }

    if !trimmed_input.contains("://") && !trimmed_input.starts_with("git@") {
        let parts: Vec<&str> = trimmed_input
            .trim_end_matches('/')
            .split('/')
            .filter(|part| !part.is_empty())
            .collect();

        return match parts.as_slice() {
            [owner, repository_name, skill_slug] => Ok(ResolvedInstallRef {
                repo_url: format!("https://github.com/{owner}/{repository_name}.git"),
                branch: None,
                selector: SkillSelector::Slug((*skill_slug).to_string()),
            }),
            [owner, repository_name, remaining_path @ ..] if remaining_path.len() > 1 => {
                Ok(ResolvedInstallRef {
                    repo_url: format!("https://github.com/{owner}/{repository_name}.git"),
                    branch: None,
                    selector: SkillSelector::Path(PathBuf::from(remaining_path.join("/"))),
                })
            }
            [_, _] => Err(anyhow!(
                "Install requires a concrete skill reference, like owner/repo/skill-slug or owner/repo/path/to/skill"
            )),
            _ => Err(anyhow!(
                "Unsupported install reference. Use an HTTPS skills.sh URL, GitHub tree URL, or owner/repo/skill reference."
            )),
        };
    }

    Err(anyhow!(
        "Unsupported install reference. Use an HTTPS skills.sh URL, GitHub tree URL, or owner/repo/skill reference."
    ))
}

fn resolve_github_tree_ref(input: &str) -> Result<Option<ResolvedInstallRef>> {
    let normalized_input = input.trim_end_matches('/');
    let prefix = "https://github.com/";

    if !normalized_input.starts_with(prefix) {
        return Ok(None);
    }

    let path = &normalized_input[prefix.len()..];
    let parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();

    if parts.len() < 3 {
        return Ok(None);
    }

    if parts[2] != "tree" {
        return Ok(None);
    }

    if parts.len() == 4 {
        return Err(anyhow!(
            "GitHub tree URLs must point to a concrete skill directory"
        ));
    }

    if parts.len() < 5 {
        return Ok(None);
    }

    let owner = parts[0];
    let repository_name = parts[1];
    let branch_name = decode_url_path_component(parts[3])?;
    let skill_path = parts[4..]
        .iter()
        .map(|segment| decode_url_path_component(segment))
        .collect::<Result<Vec<_>>>()?
        .join("/");
    let skill_path = PathBuf::from(skill_path);

    if skill_path.as_os_str().is_empty() {
        return Err(anyhow!(
            "GitHub tree URLs must point to a concrete skill directory"
        ));
    }

    Ok(Some(ResolvedInstallRef {
        repo_url: format!("https://github.com/{owner}/{repository_name}.git"),
        branch: Some(branch_name),
        selector: SkillSelector::Path(skill_path),
    }))
}

fn decode_url_path_component(component: &str) -> Result<String> {
    let bytes = component.as_bytes();
    let mut decoded = String::with_capacity(component.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err(anyhow!(
                    "Invalid percent-encoding in GitHub tree URL component: {}",
                    component
                ));
            }

            let hex = &component[index + 1..index + 3];
            let value = u8::from_str_radix(hex, 16).map_err(|_| {
                anyhow!(
                    "Invalid percent-encoding in GitHub tree URL component: {}",
                    component
                )
            })?;
            decoded.push(value as char);
            index += 3;
            continue;
        }

        decoded.push(bytes[index] as char);
        index += 1;
    }

    Ok(decoded)
}

pub fn install_skill(options: &InstallOptions, config: &Config) -> Result<InstallResult> {
    ensure_install_root_ready(&config.skills_source)?;

    let resolved_reference = resolve_install_ref(&options.reference)?;
    let temp_dir = tempfile::tempdir().context("Failed to create temporary directory")?;

    println!("Fetching skill source...");
    clone_to_path(
        &resolved_reference.repo_url,
        resolved_reference.branch.as_deref().unwrap_or(""),
        temp_dir.path(),
    )?;

    install_skill_from_checkout(temp_dir.path(), &resolved_reference, &config.skills_source)
}

pub fn install_skill_from_checkout(
    checkout_root: &Path,
    resolved_reference: &ResolvedInstallRef,
    target_root: &Path,
) -> Result<InstallResult> {
    let skill_source = resolve_skill_source(checkout_root, resolved_reference)?;
    let skill_slug = derive_skill_slug(&skill_source)?;
    let target_dir = target_root.join(&skill_slug);

    fs::create_dir_all(target_root).with_context(|| {
        format!(
            "Failed to create skills source directory at {}",
            target_root.display()
        )
    })?;

    let replaced_existing = if target_dir.exists() {
        prompt_replace_existing_skill(&skill_slug, &target_dir)?;
        true
    } else {
        false
    };

    let unique_suffix = format!(
        "{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("System clock is before UNIX_EPOCH")?
            .as_nanos()
    );
    let staging_dir = target_root.join(format!(".{}.tmp-{}", skill_slug, unique_suffix));
    let backup_dir = target_root.join(format!(".{}.bak-{}", skill_slug, unique_suffix));

    if staging_dir.exists() {
        remove_existing_path(&staging_dir)?;
    }

    if backup_dir.exists() {
        remove_existing_path(&backup_dir)?;
    }

    if let Err(error) = copy_directory_recursive(&skill_source, &staging_dir) {
        let _ = remove_existing_path(&staging_dir);
        return Err(error);
    }

    if replaced_existing {
        fs::rename(&target_dir, &backup_dir).with_context(|| {
            format!(
                "Failed to move existing skill out of the way from {} to {}",
                target_dir.display(),
                backup_dir.display()
            )
        })?;

        if let Err(error) = fs::rename(&staging_dir, &target_dir).with_context(|| {
            format!(
                "Failed to move staged skill into place from {} to {}",
                staging_dir.display(),
                target_dir.display()
            )
        }) {
            let _ = fs::rename(&backup_dir, &target_dir);
            let _ = remove_existing_path(&staging_dir);
            return Err(error);
        }

        remove_existing_path(&backup_dir)?;
    } else {
        fs::rename(&staging_dir, &target_dir).with_context(|| {
            format!(
                "Failed to move staged skill into place from {} to {}",
                staging_dir.display(),
                target_dir.display()
            )
        })?;
    }

    Ok(InstallResult {
        skill_slug,
        installed_path: target_dir,
        replaced_existing,
    })
}

fn ensure_install_root_ready(skills_source: &Path) -> Result<()> {
    if skills_source.exists() && !skills_source.is_dir() {
        return Err(anyhow!(
            "Skills source path is not a directory: {}",
            skills_source.display()
        ));
    }

    if skills_source.exists() && Repository::open(skills_source).is_ok() {
        return Err(anyhow!(
            "Skills source is currently a git repository at {}. Install requires a managed skills directory, so use a different skills_source or continue using 'capsync clone'.",
            skills_source.display()
        ));
    }

    Ok(())
}

fn resolve_skill_source(
    checkout_root: &Path,
    resolved_reference: &ResolvedInstallRef,
) -> Result<PathBuf> {
    match &resolved_reference.selector {
        SkillSelector::Path(skill_path) => resolve_checked_skill_path(checkout_root, skill_path),
        SkillSelector::Slug(skill_slug) => find_skill_directory_by_slug(checkout_root, skill_slug),
    }
}

fn resolve_checked_skill_path(checkout_root: &Path, skill_path: &Path) -> Result<PathBuf> {
    if skill_path.is_absolute()
        || skill_path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(anyhow!(
            "Skill path must be a relative path within the checkout: {}",
            skill_path.display()
        ));
    }

    let skill_root = checkout_root.join(skill_path);
    validate_skill_directory(&skill_root)?;

    let canonical_checkout_root = fs::canonicalize(checkout_root).with_context(|| {
        format!(
            "Failed to canonicalize checkout root: {}",
            checkout_root.display()
        )
    })?;
    let canonical_skill_root = fs::canonicalize(&skill_root).with_context(|| {
        format!(
            "Failed to canonicalize skill path: {}",
            skill_root.display()
        )
    })?;

    if !canonical_skill_root.starts_with(&canonical_checkout_root) {
        return Err(anyhow!(
            "Skill path escapes the checkout root: {}",
            skill_path.display()
        ));
    }

    Ok(skill_root)
}

fn validate_skill_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Skill path does not exist: {}", path.display()));
    }

    if !path.is_dir() {
        return Err(anyhow!("Skill path is not a directory: {}", path.display()));
    }

    let skill_markdown = path.join("SKILL.md");
    if !skill_markdown.exists() {
        return Err(anyhow!(
            "Expected SKILL.md in skill directory: {}",
            path.display()
        ));
    }

    Ok(())
}

fn find_skill_directory_by_slug(checkout_root: &Path, requested_slug: &str) -> Result<PathBuf> {
    let normalized_requested_slug = normalize_skill_slug(requested_slug);
    let candidates = collect_skill_directories(checkout_root)?;
    let mut matches = Vec::new();

    for candidate in candidates {
        let mut candidate_slugs = Vec::new();

        if let Some(directory_name) = candidate.file_name().and_then(|name| name.to_str()) {
            candidate_slugs.push(normalize_skill_slug(directory_name));
        }

        if let Some(skill_name) = read_skill_name(&candidate.join("SKILL.md"))? {
            candidate_slugs.push(normalize_skill_slug(&skill_name));
        }

        if candidate_slugs
            .iter()
            .any(|candidate_slug| candidate_slug == &normalized_requested_slug)
        {
            matches.push(candidate);
        }
    }

    match matches.len() {
        1 => Ok(matches.remove(0)),
        0 => Err(anyhow!(
            "Could not find a skill matching '{}' in the resolved repository",
            requested_slug
        )),
        _ => Err(anyhow!(
            "Found multiple skills matching '{}'. Use an explicit path like owner/repo/path/to/skill or a GitHub tree URL.",
            requested_slug
        )),
    }
}

fn collect_skill_directories(root: &Path) -> Result<Vec<PathBuf>> {
    let mut skill_directories = Vec::new();
    collect_skill_directories_recursive(root, &mut skill_directories)?;
    Ok(skill_directories)
}

fn collect_skill_directories_recursive(
    root: &Path,
    skill_directories: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(root)
        .with_context(|| format!("Failed to read directory {}", root.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            if entry.file_name() == ".git" {
                continue;
            }

            let skill_markdown = path.join("SKILL.md");
            if skill_markdown.exists() {
                skill_directories.push(path);
                continue;
            }

            collect_skill_directories_recursive(&path, skill_directories)?;
        }
    }

    Ok(())
}

fn read_skill_name(skill_markdown_path: &Path) -> Result<Option<String>> {
    let content = fs::read_to_string(skill_markdown_path)
        .with_context(|| format!("Failed to read {}", skill_markdown_path.display()))?;

    let mut lines = content.lines();
    if lines.next() != Some("---") {
        return Ok(None);
    }

    for line in lines {
        if line == "---" {
            break;
        }

        if let Some(value) = line.strip_prefix("name:") {
            return Ok(Some(
                value
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string(),
            ));
        }
    }

    Ok(None)
}

pub fn normalize_skill_slug(input: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for character in input.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

fn derive_skill_slug(skill_source: &Path) -> Result<String> {
    let skill_markdown_path = skill_source.join("SKILL.md");

    if let Some(skill_name) = read_skill_name(&skill_markdown_path)? {
        let normalized_name = normalize_skill_slug(&skill_name);
        if !normalized_name.is_empty() {
            return Ok(normalized_name);
        }
    }

    let directory_name = skill_source
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Cannot derive skill name from {}", skill_source.display()))?;
    let normalized_name = normalize_skill_slug(directory_name);

    if normalized_name.is_empty() {
        return Err(anyhow!(
            "Cannot derive a valid skill slug from {}",
            skill_source.display()
        ));
    }

    Ok(normalized_name)
}

fn prompt_replace_existing_skill(skill_slug: &str, target_dir: &Path) -> Result<()> {
    loop {
        print!(
            "Skill '{}' already exists at {}. Replace it? [y/N]: ",
            skill_slug,
            target_dir.display()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let normalized_input = input.trim().to_lowercase();

        if normalized_input == "y" {
            return Ok(());
        }

        if normalized_input.is_empty() || normalized_input == "n" {
            return Err(anyhow!("Aborted."));
        }

        println!("Please enter y or n.");
    }
}

fn remove_existing_path(path: &Path) -> Result<()> {
    if path.is_symlink() || path.is_file() {
        fs::remove_file(path)
            .with_context(|| format!("Failed to remove existing file at {}", path.display()))?;
    } else if path.is_dir() {
        fs::remove_dir_all(path).with_context(|| {
            format!(
                "Failed to remove existing skill directory at {}",
                path.display()
            )
        })?;
    }

    Ok(())
}

fn copy_directory_recursive(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("Failed to create directory {}", destination.display()))?;

    for entry in fs::read_dir(source)
        .with_context(|| format!("Failed to read directory {}", source.display()))?
    {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_directory_recursive(&source_path, &destination_path)?;
        } else if file_type.is_symlink() {
            return Err(anyhow!(
                "Refusing to install skills containing symlinks: {}",
                source_path.display()
            ));
        } else {
            fs::copy(&source_path, &destination_path).with_context(|| {
                format!(
                    "Failed to copy {} to {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }

    Ok(())
}
