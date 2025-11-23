# Release Guide

## Quick Release Process

### 1. Prepare Release

```bash
# Run the release script
./scripts/release.sh 0.1.0
```

This will:
- Update `Cargo.toml` version
- Update `CHANGELOG.md`
- Run tests
- Build release binary
- Commit changes
- Create git tag

### 2. Push to GitHub

```bash
git push origin master
git push origin v0.1.0
```

### 3. Automated Build

GitHub Actions will automatically:
- Build for 5 platforms:
  - Linux x86_64 (musl - static)
  - Linux ARM64 (musl - static)
  - macOS x86_64 (Intel)
  - macOS ARM64 (Apple Silicon)
  - Windows x86_64
- Create GitHub Release
- Upload all binaries

### 4. Verify Release

Check: `https://github.com/YOUR_USERNAME/socktail-rs/releases`

---

## Manual Release Process

### Platform-Specific Builds

#### Linux (current platform)
```bash
cargo build --release
```

#### Linux (static musl)
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

#### All platforms (requires cross-compilation tools)
```bash
./scripts/build-all.sh
```

**Note**: Local cross-compilation from Linux to macOS/Windows requires additional toolchains. For full multi-platform builds, use GitHub Actions (automated on tag push) or install `cross`:

```bash
cargo install cross --git https://github.com/cross-rs/cross
# Then modify build-all.sh to use 'cross' instead of 'cargo'
```

### Test Build

```bash
./scripts/test-build.sh
```

---

## Release Checklist

### Before Release
- [ ] All tests passing (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Documentation updated
- [ ] README.md accurate

### After Release
- [ ] GitHub Release created
- [ ] All platform binaries uploaded
- [ ] Release notes complete
- [ ] Tested download links
- [ ] Announced (if applicable)

---

## Platform Build Matrix

| Platform | Target | Static | Size (est.) |
|----------|--------|--------|-------------|
| Linux x86_64 | x86_64-unknown-linux-musl | ✅ Yes | ~1.9 MB |
| Linux ARM64 | aarch64-unknown-linux-musl | ✅ Yes | ~2.0 MB |
| macOS x86_64 | x86_64-apple-darwin | ❌ No | ~1.7 MB |
| macOS ARM64 | aarch64-apple-darwin | ❌ No | ~1.6 MB |
| Windows x86_64 | x86_64-pc-windows-msvc | ❌ No | ~1.5 MB |

---

## Version Naming

Follow Semantic Versioning (SemVer):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality (backwards-compatible)
- **PATCH**: Bug fixes (backwards-compatible)

Examples:
- `0.1.0` - Initial release
- `0.1.1` - Bug fix
- `0.2.0` - New feature (minor)
- `1.0.0` - Stable release (major)

---

## GitHub Actions Workflows

### Release Workflow
**Trigger**: Git tag push (e.g., `v0.1.0`)
**File**: `.github/workflows/release.yml`
**Duration**: ~15-20 minutes
**Output**: GitHub Release with binaries

### CI Workflow
**Trigger**: Push to main, PR
**File**: `.github/workflows/ci.yml`
**Duration**: ~5-10 minutes
**Output**: Test results

---

## Troubleshooting

### Build Fails on GitHub Actions

Check:
1. Rust version compatibility
2. Dependencies availability
3. Cross-compilation tools
4. Workflow logs

### Binary Too Large

Optimize:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
strip = true        # Strip symbols
```

### Static Linking Issues

For fully static builds, use musl targets:
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-musl`

---

## Custom Builds

### With Embedded Auth Key

```bash
AUTH_KEY=tskey-auth-xxxxx cargo build --release
```

### With Control Server

```bash
AUTH_KEY=tskey-auth-xxxxx \
CONTROL_URL=https://headscale.example.com \
cargo build --release
```

### Using Build Script

```bash
AUTH_KEY=tskey-auth-xxxxx ./scripts/build-all.sh
```

---

## Distribution Checklist

When distributing binaries:
- [ ] Include LICENSE file
- [ ] Include README.md
- [ ] Test binary on clean system
- [ ] Verify no dynamic dependencies (for static builds)
- [ ] Check binary size
- [ ] Test all command-line options
- [ ] Verify version matches (`socktail --version`)

---

## Publishing to crates.io

```bash
# Login (one time)
cargo login

# Publish
cargo publish

# Dry run first
cargo publish --dry-run
```

---

## Quick Commands Reference

```bash
# Test build
./scripts/test-build.sh

# Full release
./scripts/release.sh 0.1.0

# Manual build all platforms
./scripts/build-all.sh

# Check version
socktail --version

# Run tests
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```
