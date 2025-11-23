#!/usr/bin/env bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
AUTH_KEY="${AUTH_KEY:-}"
CONTROL_URL="${CONTROL_URL:-}"

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}  SockTail Cross-Platform Build${NC}"
echo -e "${BLUE}  Version: ${VERSION}${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Create dist directory
mkdir -p dist

# Define targets
declare -A TARGETS=(
    ["x86_64-unknown-linux-musl"]="Linux x86_64 (musl)"
    ["aarch64-unknown-linux-musl"]="Linux ARM64 (musl)"
    ["x86_64-apple-darwin"]="macOS x86_64 (Intel)"
    ["aarch64-apple-darwin"]="macOS ARM64 (Apple Silicon)"
    ["x86_64-pc-windows-gnu"]="Windows x86_64"
)

# Build function
build_target() {
    local target=$1
    local description=$2

    echo -e "${YELLOW}Building for ${description}...${NC}"

    # Set environment variables for build
    if [ -n "$AUTH_KEY" ]; then
        export AUTH_KEY
    fi
    if [ -n "$CONTROL_URL" ]; then
        export CONTROL_URL
    fi

    # Build
    if cargo build --release --target "$target" 2>&1; then
        echo -e "${GREEN}✅ Built ${description}${NC}"

        # Determine binary name and extension
        local binary_name="socktail"
        local archive_name="socktail-v${VERSION}-${target}"

        if [[ $target == *"windows"* ]]; then
            binary_name="socktail.exe"

            # Copy and compress
            cp "target/${target}/release/${binary_name}" "dist/${archive_name}.exe"

            # Create zip
            cd dist
            zip -q "${archive_name}.zip" "${archive_name}.exe"
            rm "${archive_name}.exe"
            cd ..

            echo -e "${GREEN}   📦 Created: dist/${archive_name}.zip${NC}"
        else
            # Copy binary
            cp "target/${target}/release/${binary_name}" "dist/${archive_name}"

            # Strip if available
            if command -v strip &> /dev/null; then
                strip "dist/${archive_name}" 2>/dev/null || true
            fi

            # Create tar.gz
            cd dist
            tar czf "${archive_name}.tar.gz" "${archive_name}"
            rm "${archive_name}"
            cd ..

            echo -e "${GREEN}   📦 Created: dist/${archive_name}.tar.gz${NC}"
        fi

        # Show size
        if [[ $target == *"windows"* ]]; then
            local size=$(ls -lh "dist/${archive_name}.zip" | awk '{print $5}')
            echo -e "${BLUE}   📊 Size: ${size}${NC}"
        else
            local size=$(ls -lh "dist/${archive_name}.tar.gz" | awk '{print $5}')
            echo -e "${BLUE}   📊 Size: ${size}${NC}"
        fi

        return 0
    else
        echo -e "${RED}❌ Failed to build ${description}${NC}"
        return 1
    fi
}

# Track successes and failures
declare -a SUCCESSES
declare -a FAILURES

# Build all targets
for target in "${!TARGETS[@]}"; do
    echo ""
    if build_target "$target" "${TARGETS[$target]}"; then
        SUCCESSES+=("${TARGETS[$target]}")
    else
        FAILURES+=("${TARGETS[$target]}")
    fi
done

# Summary
echo ""
echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}  Build Summary${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

if [ ${#SUCCESSES[@]} -gt 0 ]; then
    echo -e "${GREEN}✅ Successful builds (${#SUCCESSES[@]}):${NC}"
    for platform in "${SUCCESSES[@]}"; do
        echo -e "   ${GREEN}•${NC} $platform"
    done
fi

if [ ${#FAILURES[@]} -gt 0 ]; then
    echo ""
    echo -e "${RED}❌ Failed builds (${#FAILURES[@]}):${NC}"
    for platform in "${FAILURES[@]}"; do
        echo -e "   ${RED}•${NC} $platform"
    done
fi

echo ""
echo -e "${BLUE}📦 Release artifacts in: dist/${NC}"
ls -lh dist/

echo ""
if [ ${#FAILURES[@]} -eq 0 ]; then
    echo -e "${GREEN}🎉 All builds completed successfully!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some builds failed. Check logs above.${NC}"
    exit 1
fi
