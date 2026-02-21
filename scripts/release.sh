#!/usr/bin/env bash

# Release Process
# 1. Run release script to open a PR:
#   ```bash
#   ./scripts/release.sh create_pr 1.0.1
#   ```
# 2. Merge the PR
# 3. Create and push the tag:
#   ```bash
#   ./scripts/release.sh create_tag 1.0.1 --publish-crates
#   ```

set -e

usage() {
    echo "Usage: $0 <function_name> <version> [--publish-crates|-p]"
    echo ""
    echo "Available functions: create_pr, create_tag, delete_tag"
    echo ""
    echo "Flags: --publish-crates, -p (only valid with create_tag)"
    echo ""
    echo "Example: $0 create_pr 1.0.0"
    echo "Example: $0 create_tag 1.0.0 --publish-crates"
}

POSITIONAL=()
PUBLISH_CRATES=false

for arg in "$@"; do
    case "$arg" in
        --publish-crates|-p)
            PUBLISH_CRATES=true
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        -* )
            echo "Error: Unknown flag '$arg'"
            usage
            exit 1
            ;;
        * )
            POSITIONAL+=("$arg")
            ;;
    esac
done

if [[ ${#POSITIONAL[@]} -ne 2 ]]; then
    usage
    exit 1
fi

FUNCTION="${POSITIONAL[0]}"
VERSION="${POSITIONAL[1]}"

# Validate version format (semver)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9\.]+)?$ ]]; then
    echo "Error: Invalid version format '$VERSION'"
    echo "Version must follow semantic versioning: X.Y.Z or X.Y.Z-suffix"
    exit 1
fi

validate_changelog_for_pr() {
    if ! grep -q "## \[Unreleased\]" CHANGELOG.md; then
        echo "Error: CHANGELOG.md doesn't contain an [Unreleased] section"
        echo "Please update CHANGELOG.md before releasing"
        exit 1
    fi

    if grep -q "\[$VERSION\]" CHANGELOG.md; then
        echo "Error: Version $VERSION already exists in CHANGELOG.md"
        echo "This release appears to already be documented"
        exit 1
    fi
}

validate_changelog_for_tag() {
    if ! grep -q "## \[Unreleased\]" CHANGELOG.md; then
        echo "Error: CHANGELOG.md doesn't contain an [Unreleased] section"
        exit 1
    fi

    if ! grep -q "\[$VERSION\]" CHANGELOG.md; then
        echo "Error: Version $VERSION not found in CHANGELOG.md"
        echo "Please merge the release PR before tagging"
        exit 1
    fi
}

update_version_files() {
    DATE=$(date +%Y-%m-%d)

    if ! grep -q "## \[Unreleased\]" CHANGELOG.md; then
        echo "Error: CHANGELOG.md doesn't contain expected '## [Unreleased]' section"
        exit 1
    fi

    sed -i.bak \
        -e "s/^version = .*/version = \"$VERSION\"/" \
        Cargo.toml
    rm Cargo.toml.bak

    sed -i.bak "s/## \[Unreleased\]$/## [Unreleased]$'\n\n'## [$VERSION] - $DATE/" CHANGELOG.md
    rm CHANGELOG.md.bak

    echo "Updated version to $VERSION in Cargo.toml and CHANGELOG.md"
}

ensure_clean_master() {
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$CURRENT_BRANCH" != "master" ]]; then
        echo "Error: You must be on the master branch"
        exit 1
    fi

    if ! git diff-index --quiet HEAD --; then
        echo "Error: You have uncommitted changes"
        echo "Please commit or stash them before releasing"
        exit 1
    fi
}

create_pr() {
    ensure_clean_master
    validate_changelog_for_pr

    RELEASE_BRANCH="release/v$VERSION"
    if git show-ref --quiet "refs/heads/$RELEASE_BRANCH"; then
        echo "Error: Branch $RELEASE_BRANCH already exists locally"
        exit 1
    fi

    if git ls-remote --exit-code --heads origin "$RELEASE_BRANCH" >/dev/null 2>&1; then
        echo "Error: Branch $RELEASE_BRANCH already exists on origin"
        exit 1
    fi

    git checkout -b "$RELEASE_BRANCH"

    update_version_files

    git add Cargo.toml CHANGELOG.md
    git commit -m "release(CapSync): version $VERSION [skip ci]"
    git push -u origin "$RELEASE_BRANCH"

    if command -v gh >/dev/null 2>&1; then
        # Get authenticated username
        GH_USER=$(gh auth whoami 2>/dev/null || echo "")

        gh pr create \
            --title "release(CapSync): version $VERSION" \
            --body "## Summary
- Bump versions to $VERSION
- Update CHANGELOG.md

## Next steps
- Merge this PR
- Run ./scripts/release.sh create_tag $VERSION [--publish-crates]" \
            --head "$RELEASE_BRANCH" \
            --base "master" \
            --assignee "${GH_USER:-@me}" \
            --label "release" \
            --milestone "$VERSION"
    else
        echo "Release branch pushed: $RELEASE_BRANCH"
        echo "Open a PR targeting master and include:"
        echo "- Bump versions to $VERSION"
        echo "- Update CHANGELOG.md"
    fi

    echo "Release PR ready for review."
}

create_tag() {
    ensure_clean_master
    validate_changelog_for_tag

    echo "This will create and push tag v$VERSION, triggering the release workflow."
    read -r -p "Continue? (y/N): " confirm
    if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
        echo "Operation cancelled"
        exit 0
    fi

    echo "Creating tag v$VERSION..."
    git tag -a "v$VERSION" --message "CapSync v$VERSION"
    git push origin "v$VERSION"

    if [[ "$PUBLISH_CRATES" == "true" ]]; then
        echo "Publishing to crates.io..."
        cargo publish -p capsync
        echo "Publish to crates.io complete."
    fi

    echo "Tag v$VERSION created and pushed!"
    echo "The release workflow should start automatically. Monitor progress at:"
    echo "https://github.com/pixincreate/CapSync/actions"
}

delete_tag() {
    if [[ "$PUBLISH_CRATES" == "true" ]]; then
        echo "Error: --publish-crates is only supported with create_tag"
        exit 1
    fi

    echo "Warning: This will delete tag v$VERSION locally and remotely."
    read -r -p "Continue? (y/N): " confirm
    if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
        echo "Operation cancelled"
        exit 0
    fi

    echo "Deleting tag v$VERSION..."
    git tag -d "v$VERSION" 2>/dev/null || echo "Tag v$VERSION not found locally"
    git push --delete origin "v$VERSION" 2>/dev/null || echo "Tag v$VERSION not found on remote"
    echo "Tag v$VERSION deleted!"
}

# Check if function exists
if [[ "$FUNCTION" != "create_pr" && "$FUNCTION" != "create_tag" && "$FUNCTION" != "delete_tag" ]]; then
    echo "Error: Unknown function '$FUNCTION'"
    echo "Available functions: create_pr, create_tag, delete_tag"
    exit 1
fi

eval "$FUNCTION"
