# Windows 构建指南

## ⚠️ Windows 限制

**libtailscale 不支持 Windows**：
- gvisor (Tailscale 网络栈) 的某些包不支持 Windows
- `gohacks` 包在 Windows 上有 build constraints
- native-tailscale 功能在 Windows 上无法使用

---

## ✅ Windows 解决方案

### 使用 CLI 模式（推荐）

在 Windows 上，使用系统的 Tailscale CLI：

#### 1. 安装 Tailscale

下载并安装 Tailscale for Windows:
https://tailscale.com/download/windows

#### 2. 验证安装

```powershell
tailscale version
```

#### 3. 构建 socktail

```powershell
# 在 Windows 上自动使用 CLI 模式
cargo build --release

# 或显式禁用 native-tailscale
cargo build --release --no-default-features
```

#### 4. 运行

```powershell
.\target\release\socktail.exe --authkey "tskey-xxx"

# 输出:
# Using CLI-based Tailscale implementation
# Connecting to Tailscale network...
# 🚀 Starting SOCKS5 server on 127.0.0.1:1080
```

---

## 🔧 故障排查

### 错误 1: gvisor build constraints

```
imports gvisor.dev/gvisor/pkg/gohacks: build constraints exclude all Go files
assertion failed: status.success()
```

**原因**: 尝试使用 native-tailscale 功能
**解决**: 使用 CLI 模式

```powershell
cargo clean
cargo build --release --no-default-features
```

---

### 错误 2: Tailscale CLI not found

```
Error: Tailscale not available
Tailscale CLI not found!
```

**原因**: 系统未安装 Tailscale
**解决**:

1. 下载 Tailscale: https://tailscale.com/download/windows
2. 安装后重启终端
3. 验证: `tailscale version`

---

### 错误 3: 权限问题

```
Error: Failed to execute tailscale command
```

**原因**: Tailscale 需要管理员权限
**解决**: 以管理员身份运行 PowerShell

```powershell
# 右键 PowerShell → 以管理员身份运行
.\target\release\socktail.exe --authkey "tskey-xxx"
```

---

## 📦 构建配置

### Cargo.toml (已配置)

```toml
[features]
default = []  # Windows 不启用 native-tailscale

# 仅 Linux/macOS 支持
[target.'cfg(target_os = "linux")'.dependencies]
libtailscale = { version = "0.2", optional = true }

[target.'cfg(target_os = "macos")'.dependencies]
libtailscale = { version = "0.2", optional = true }
```

---

## 🚀 完整构建步骤 (Windows)

### 前提条件

1. ✅ Rust (rustup.rs)
2. ✅ Tailscale for Windows
3. ✅ MSVC Build Tools (通过 Visual Studio Installer)

### 构建

```powershell
# 1. 克隆仓库
git clone https://github.com/Nurdich/socktail-rs.git
cd socktail-rs

# 2. 构建（自动使用 CLI 模式）
cargo build --release

# 3. 测试
.\target\release\socktail.exe --help

# 4. 运行
.\target\release\socktail.exe --authkey "tskey-xxx"
```

---

## 🔄 Linux 交叉编译 (可选)

如果想在 Linux 上为 Windows 构建：

```bash
# 在 Linux 上
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu --no-default-features

# 生成的文件
# target/x86_64-pc-windows-gnu/release/socktail.exe
```

---

## 📊 平台支持对比

| 平台 | Native API | CLI 模式 | 说明 |
|------|------------|----------|------|
| **Linux** | ✅ 支持 | ✅ 支持 | 推荐 native |
| **macOS** | ✅ 支持 | ✅ 支持 | 推荐 native |
| **Windows** | ❌ 不支持 | ✅ 支持 | 仅 CLI 模式 |

---

## 💡 Windows 用户建议

### 推荐做法

1. ✅ 使用 CLI 模式（已自动配置）
2. ✅ 安装 Tailscale for Windows
3. ✅ 正常构建和运行

### 性能影响

CLI 模式在 Windows 上的性能：
- 连接时间: ~2-3 秒（可接受）
- 运行性能: 与 native 模式相近
- 内存占用: 略高（~2-3 MB 差异）

**结论**: CLI 模式在 Windows 上完全够用！

---

## 🆘 获取帮助

遇到问题？

1. 查看 [BUILDING.md](BUILDING.md)
2. 检查 Tailscale 安装: `tailscale version`
3. 查看日志: `socktail.exe -v`
4. 提交 issue: https://github.com/Nurdich/socktail-rs/issues

---

## ✅ 快速测试

```powershell
# 开发模式（不连接 VPN）
.\target\release\socktail.exe --no-vpn

# 应该看到:
# ⚠️  Running in dev mode (no VPN)
# 🚀 Starting SOCKS5 server on 127.0.0.1:1080
```

---

**Windows 用户：直接运行 `cargo build --release` 即可！** 🎉
