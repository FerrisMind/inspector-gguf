# Inspector GGUF Deployment Guide

This guide covers building, packaging, and deploying Inspector GGUF across different platforms and environments.

## 🏗️ Build Requirements

### System Requirements

#### Minimum Requirements
- **Rust**: 1.70 or newer
- **Memory**: 2GB RAM for building
- **Storage**: 1GB free space for build artifacts
- **Network**: Internet connection for dependency downloads

#### Recommended Requirements
- **Rust**: Latest stable version
- **Memory**: 8GB RAM for faster builds
- **Storage**: 5GB free space for multiple targets
- **CPU**: Multi-core processor for parallel compilation

### Platform-Specific Requirements

#### Windows
```powershell
# Install Rust
winget install Rustlang.Rust.GNU

# Install Visual Studio Build Tools (required for linking)
winget install Microsoft.VisualStudio.2022.BuildTools

# Install Git
winget install Git.Git
```

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (optional, for additional tools)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

#### Linux (Ubuntu/Debian)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build dependencies
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev libgtk-3-dev

# For older systems, you might need additional packages
sudo apt install -y cmake libfontconfig1-dev
```

#### Linux (CentOS/RHEL/Fedora)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build dependencies
sudo dnf install -y gcc gcc-c++ openssl-devel gtk3-devel

# Or for older versions
sudo yum install -y gcc gcc-c++ openssl-devel gtk3-devel
```

## 🔨 Building from Source

### Development Build

#### Quick Development Build
```bash
# Clone repository
git clone https://github.com/FerrisMind/inspector-gguf
cd inspector-gguf

# Build in debug mode (faster compilation)
cargo build

# Run with GUI
cargo run -- --gui

# Run tests
cargo test
```

#### Development with Hot Reload
```bash
# Install cargo-watch for automatic rebuilds
cargo install cargo-watch

# Watch for changes and rebuild
cargo watch -x "run -- --gui"
```

### Production Build

#### Optimized Release Build
```bash
# Build with full optimizations
cargo build --release

# The binary will be in target/release/
ls -la target/release/inspector-gguf*
```

#### Build with Profiling Support
```bash
# Build with profiling enabled
cargo build --profile profiling

# Run with profiling
./target/profiling/inspector-gguf --profile
```

### Cross-Platform Building

#### Setup Cross-Compilation
```bash
# Install cross-compilation tool
cargo install cross

# Add target platforms
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
```

#### Build for Multiple Targets
```bash
# Windows (from Linux/macOS)
cross build --release --target x86_64-pc-windows-gnu

# macOS Intel (from Linux/Windows)
cross build --release --target x86_64-apple-darwin

# macOS Apple Silicon (from Linux/Windows)
cross build --release --target aarch64-apple-darwin

# Linux (from Windows/macOS)
cross build --release --target x86_64-unknown-linux-gnu
```

## 📦 Packaging

### Windows Packaging

#### Create Windows Installer
```powershell
# Install WiX Toolset
winget install WiXToolset.WiX

# Build MSI installer (requires WiX configuration)
# See packaging/windows/installer.wxs for configuration
```

#### Portable Windows Package
```powershell
# Create portable package
mkdir inspector-gguf-windows-portable
copy target\release\inspector-gguf.exe inspector-gguf-windows-portable\
copy README.md inspector-gguf-windows-portable\
copy LICENSE inspector-gguf-windows-portable\

# Create ZIP archive
Compress-Archive -Path inspector-gguf-windows-portable -DestinationPath inspector-gguf-windows-portable.zip
```

### macOS Packaging

#### Create macOS App Bundle
```bash
# Create app bundle structure
mkdir -p Inspector-GGUF.app/Contents/MacOS
mkdir -p Inspector-GGUF.app/Contents/Resources

# Copy binary
cp target/release/inspector-gguf Inspector-GGUF.app/Contents/MacOS/

# Create Info.plist
cat > Inspector-GGUF.app/Contents/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>inspector-gguf</string>
    <key>CFBundleIdentifier</key>
    <string>com.ferrismind.inspector-gguf</string>
    <key>CFBundleName</key>
    <string>Inspector GGUF</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
</dict>
</plist>
EOF

# Create DMG (requires create-dmg)
brew install create-dmg
create-dmg --volname "Inspector GGUF" --window-pos 200 120 --window-size 600 300 --icon-size 100 --icon "Inspector-GGUF.app" 175 120 --hide-extension "Inspector-GGUF.app" --app-drop-link 425 120 "Inspector-GGUF.dmg" "Inspector-GGUF.app"
```

### Linux Packaging

#### Create DEB Package (Debian/Ubuntu)
```bash
# Install packaging tools
sudo apt install -y devscripts debhelper

# Create package structure
mkdir -p inspector-gguf-deb/DEBIAN
mkdir -p inspector-gguf-deb/usr/bin
mkdir -p inspector-gguf-deb/usr/share/doc/inspector-gguf

# Copy files
cp target/release/inspector-gguf inspector-gguf-deb/usr/bin/
cp README.md inspector-gguf-deb/usr/share/doc/inspector-gguf/
cp LICENSE inspector-gguf-deb/usr/share/doc/inspector-gguf/

# Create control file
cat > inspector-gguf-deb/DEBIAN/control << EOF
Package: inspector-gguf
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: FerrisMind <contact@ferrismind.com>
Description: A powerful GGUF file inspection tool
 Inspector GGUF provides both graphical and command-line interfaces
 for analyzing GGUF (GPT-Generated Unified Format) files used in
 machine learning applications.
EOF

# Build DEB package
dpkg-deb --build inspector-gguf-deb inspector-gguf_0.1.0_amd64.deb
```

#### Create RPM Package (CentOS/RHEL/Fedora)
```bash
# Install packaging tools
sudo dnf install -y rpm-build rpmdevtools

# Setup RPM build environment
rpmdev-setuptree

# Create spec file
cat > ~/rpmbuild/SPECS/inspector-gguf.spec << EOF
Name:           inspector-gguf
Version:        0.1.0
Release:        1%{?dist}
Summary:        A powerful GGUF file inspection tool

License:        MIT
URL:            https://github.com/FerrisMind/inspector-gguf
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust cargo
Requires:       gtk3

%description
Inspector GGUF provides both graphical and command-line interfaces
for analyzing GGUF (GPT-Generated Unified Format) files used in
machine learning applications.

%prep
%setup -q

%build
cargo build --release

%install
mkdir -p %{buildroot}%{_bindir}
cp target/release/inspector-gguf %{buildroot}%{_bindir}/

%files
%{_bindir}/inspector-gguf
%doc README.md
%license LICENSE

%changelog
* Wed Jan 01 2025 FerrisMind <contact@ferrismind.com> - 0.1.0-1
- Initial package
EOF

# Build RPM
rpmbuild -ba ~/rpmbuild/SPECS/inspector-gguf.spec
```

#### Create AppImage (Universal Linux)
```bash
# Install AppImage tools
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage

# Create AppDir structure
mkdir -p Inspector-GGUF.AppDir/usr/bin
mkdir -p Inspector-GGUF.AppDir/usr/share/applications
mkdir -p Inspector-GGUF.AppDir/usr/share/icons/hicolor/256x256/apps

# Copy files
cp target/release/inspector-gguf Inspector-GGUF.AppDir/usr/bin/
cp assets/icons/256x256.png Inspector-GGUF.AppDir/usr/share/icons/hicolor/256x256/apps/inspector-gguf.png

# Create desktop file
cat > Inspector-GGUF.AppDir/usr/share/applications/inspector-gguf.desktop << EOF
[Desktop Entry]
Type=Application
Name=Inspector GGUF
Comment=A powerful GGUF file inspection tool
Exec=inspector-gguf
Icon=inspector-gguf
Categories=Development;Utility;
EOF

# Create AppRun
cat > Inspector-GGUF.AppDir/AppRun << 'EOF'
#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")"
exec "${HERE}/usr/bin/inspector-gguf" "$@"
EOF
chmod +x Inspector-GGUF.AppDir/AppRun

# Build AppImage
./appimagetool-x86_64.AppImage Inspector-GGUF.AppDir Inspector-GGUF-x86_64.AppImage
```

## 🚀 Deployment Strategies

### GitHub Releases

#### Automated Release with GitHub Actions
```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Build
      run: cargo build --release
    
    - name: Package
      run: |
        if [[ "${{ matrix.os }}" == "windows-latest" ]]; then
          7z a inspector-gguf-windows.zip target/release/inspector-gguf.exe README.md LICENSE
        elif [[ "${{ matrix.os }}" == "macos-latest" ]]; then
          tar -czf inspector-gguf-macos.tar.gz -C target/release inspector-gguf README.md LICENSE
        else
          tar -czf inspector-gguf-linux.tar.gz -C target/release inspector-gguf README.md LICENSE
        fi
    
    - name: Release
      uses: softprops/action-gh-release@v1
      with:
        files: inspector-gguf-*
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### Container Deployment

#### Docker Container
```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libgtk-3-0 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/inspector-gguf /usr/local/bin/

ENTRYPOINT ["inspector-gguf"]
```

#### Build and Run Container
```bash
# Build container
docker build -t inspector-gguf .

# Run with GUI (requires X11 forwarding)
docker run -it --rm \
  -e DISPLAY=$DISPLAY \
  -v /tmp/.X11-unix:/tmp/.X11-unix \
  -v $(pwd)/models:/models \
  inspector-gguf --gui

# Run CLI mode
docker run -it --rm \
  -v $(pwd)/models:/models \
  inspector-gguf /models/model.gguf
```

### Distribution Channels

#### Crates.io Publication
```bash
# Login to crates.io
cargo login

# Publish package
cargo publish

# Verify publication
cargo search inspector-gguf
```

#### Package Managers

##### Homebrew (macOS/Linux)
```ruby
# Formula for Homebrew
class InspectorGguf < Formula
  desc "A powerful GGUF file inspection tool"
  homepage "https://github.com/FerrisMind/inspector-gguf"
  url "https://github.com/FerrisMind/inspector-gguf/archive/v0.1.0.tar.gz"
  sha256 "..."
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/inspector-gguf", "--help"
  end
end
```

##### Chocolatey (Windows)
```xml
<!-- inspector-gguf.nuspec -->
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>inspector-gguf</id>
    <version>0.1.0</version>
    <title>Inspector GGUF</title>
    <authors>FerrisMind</authors>
    <description>A powerful GGUF file inspection tool with graphical and command-line interface</description>
    <projectUrl>https://github.com/FerrisMind/inspector-gguf</projectUrl>
    <licenseUrl>https://github.com/FerrisMind/inspector-gguf/blob/main/LICENSE</licenseUrl>
    <tags>gguf llm model inspector rust</tags>
  </metadata>
  <files>
    <file src="target\release\inspector-gguf.exe" target="tools\inspector-gguf.exe" />
  </files>
</package>
```

## 🔧 Configuration Management

### Build Configuration

#### Cargo.toml Optimization
```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = true
overflow-checks = false
panic = "unwind"
incremental = false
debug = false
debug-assertions = false
rpath = false
```

#### Feature Flags
```toml
[features]
default = ["gui"]
gui = ["eframe", "egui"]
cli-only = []
profiling = ["puffin", "puffin_http"]
```

### Environment Configuration

#### Development Environment
```bash
# .env file for development
RUST_LOG=debug
RUST_BACKTRACE=1
INSPECTOR_GGUF_DEV=true
```

#### Production Environment
```bash
# Production configuration
RUST_LOG=warn
INSPECTOR_GGUF_PROFILE=false
INSPECTOR_GGUF_UPDATE_CHECK=true
```

## 📊 Performance Optimization

### Build Performance

#### Parallel Compilation
```bash
# Use all CPU cores for compilation
export CARGO_BUILD_JOBS=$(nproc)

# Or set specific number of jobs
export CARGO_BUILD_JOBS=8
```

#### Incremental Builds
```bash
# Enable incremental compilation for development
export CARGO_INCREMENTAL=1

# Disable for release builds (already done in Cargo.toml)
cargo build --release
```

#### Link-Time Optimization
```toml
# In Cargo.toml
[profile.release]
lto = "thin"  # or "fat" for maximum optimization
```

### Runtime Performance

#### Memory Optimization
- Use `jemalloc` allocator for better memory management
- Profile memory usage with `valgrind` or `heaptrack`
- Implement memory pooling for frequent allocations

#### I/O Optimization
- Use async I/O for file operations
- Implement streaming for large files
- Cache frequently accessed data

## 🧪 Testing in Deployment

### Automated Testing

#### Unit Tests
```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

#### Integration Tests
```bash
# Run integration tests only
cargo test --test integration

# Run with specific features
cargo test --features gui
```

#### End-to-End Tests
```bash
# Test GUI functionality (requires display)
cargo test --test e2e_gui

# Test CLI functionality
cargo test --test e2e_cli
```

### Manual Testing Checklist

#### Pre-Release Testing
- [ ] Build on all target platforms
- [ ] Test with various GGUF file sizes
- [ ] Verify all export formats work
- [ ] Test language switching
- [ ] Verify update checking
- [ ] Test drag-and-drop functionality
- [ ] Validate error handling

#### Post-Release Verification
- [ ] Download and test release binaries
- [ ] Verify package manager installations
- [ ] Test on clean systems
- [ ] Validate documentation accuracy

## 🚨 Troubleshooting Deployment

### Common Build Issues

#### Rust Version Conflicts
```bash
# Update Rust toolchain
rustup update stable

# Check version
rustc --version
```

#### Missing Dependencies
```bash
# Linux: Install system dependencies
sudo apt install build-essential pkg-config libssl-dev

# macOS: Install Xcode tools
xcode-select --install

# Windows: Install Visual Studio Build Tools
```

#### Cross-Compilation Issues
```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Install cross-compilation tools
cargo install cross

# Use cross instead of cargo
cross build --target x86_64-pc-windows-gnu
```

### Runtime Issues

#### GUI Not Starting
- Check display environment variables
- Verify graphics drivers
- Test with `--cli` mode first

#### File Loading Failures
- Verify file permissions
- Check available memory
- Test with smaller files

#### Export Failures
- Check write permissions
- Verify disk space
- Test output directory access

## 📋 Deployment Checklist

### Pre-Deployment
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Version numbers incremented
- [ ] Changelog updated
- [ ] Security audit completed

### Deployment Process
- [ ] Build for all target platforms
- [ ] Create release packages
- [ ] Upload to distribution channels
- [ ] Update package managers
- [ ] Announce release

### Post-Deployment
- [ ] Monitor for issues
- [ ] Respond to user feedback
- [ ] Plan next release cycle
- [ ] Update documentation

---

This deployment guide ensures consistent, reliable builds and deployments across all supported platforms and distribution channels.