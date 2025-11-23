#!/usr/bin/env bash
# Release script - Creates a new release
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}  SockTail Release Script${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check if version is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version not specified${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

# Validate version format
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version must be in format: X.Y.Z (e.g., 0.1.0)"
    exit 1
fi

echo -e "${YELLOW}Preparing release ${TAG}${NC}"
echo ""

# Check git status
if [ -n "$(git status --porcelain)" ]; then
    echo -e "${RED}Error: Working directory is not clean${NC}"
    echo "Please commit or stash your changes first"
    exit 1
fi

# Update version in Cargo.toml
echo -e "${BLUE}📝 Updating Cargo.toml...${NC}"
sed -i "s/^version = .*/version = \"${VERSION}\"/" Cargo.toml

# Update CHANGELOG.md
echo -e "${BLUE}📝 Updating CHANGELOG.md...${NC}"
DATE=$(date +%Y-%m-%d)
sed -i "s/## \[Unreleased\]/## [Unreleased]\n\n## [${VERSION}] - ${DATE}/" CHANGELOG.md

# Run tests
echo -e "${BLUE}🧪 Running tests...${NC}"
cargo test --all-features

# Build release binary
echo -e "${BLUE}🔨 Building release binary...${NC}"
cargo build --release

# Commit changes
echo -e "${BLUE}📦 Committing version bump...${NC}"
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "Bump version to ${VERSION}"

# Create git tag
echo -e "${BLUE}🏷️  Creating git tag ${TAG}...${NC}"
git tag -a "${TAG}" -m "Release ${TAG}"

echo ""
echo -e "${GREEN}✅ Release ${TAG} prepared successfully!${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Review the changes:"
echo -e "   ${BLUE}git show${NC}"
echo ""
echo "2. Push to GitHub (this will trigger the release build):"
echo -e "   ${BLUE}git push origin master${NC}"
echo -e "   ${BLUE}git push origin ${TAG}${NC}"
echo ""
echo "3. GitHub Actions will automatically:"
echo "   • Build for all platforms"
echo "   • Create GitHub Release"
echo "   • Upload all binaries"
echo ""
echo "4. After successful build, check:"
echo "   https://github.com/YOUR_USERNAME/socktail-rs/releases/tag/${TAG}"
echo ""
