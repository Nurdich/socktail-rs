# 使用 libtailscale-rs 构建指南

## ✅ 当前方案

使用官方 `libtailscale-rs` crate 进行 Tailscale 集成。

### 为什么需要 Go？

```
libtailscale (Tailscale 官方库)
    ↓ 用 Go 语言编写
    ↓
libtailscale-rs (Rust 绑定)
    ↓ 编译时调用 Go 构建 libtailscale
    ↓
socktail (我们的 Rust 应用)
```

**libtailscale 本身是 Go 实现**，所以构建时需要 Go 编译器。

---

## 📋 构建要求

### 必需依赖

1. **Rust** 1.70+
   ```bash
   rustc --version
   ```

2. **Go** 1.20+
   ```bash
   go version
   ```

3. **构建工具**
   - Linux: `gcc`, `make`
   - macOS: Xcode Command Line Tools
   - Windows: MSVC 或 MinGW

---

## 🔨 构建步骤

### 方法 1: 标准构建（需要 Go）

```bash
# 1. 确保 Go 已安装
go version

# 2. 构建（首次会下载并编译 libtailscale，约 3-5 分钟）
cargo build --release

# 3. 运行
./target/release/socktail --authkey "tskey-xxx"
```

**首次构建**:
- 自动克隆 libtailscale 源码
- 使用 Go 编译 libtailscale
- 链接到 Rust 二进制
- 总时间: ~3-5 分钟

**后续构建**:
- 使用缓存的 libtailscale
- 只编译 Rust 代码
- 总时间: ~30 秒

---

### 方法 2: 无 Go 构建（CLI 模式）

如果没有 Go 环境：

```bash
# 构建不带 native-tailscale 的版本
cargo build --release --no-default-features

# 需要系统安装 tailscale CLI
tailscale version

# 运行
./target/release/socktail --authkey "tskey-xxx"
```

---

## 🌐 构建问题排查

### 问题 1: 网络访问失败

**错误**:
```
dial tcp: lookup storage.googleapis.com: connection refused
```

**原因**: Go 需要下载依赖包，但网络不通

**解决方案**:

#### 方案 A: 配置 Go 代理
```bash
# 使用 Go 代理
export GOPROXY=https://goproxy.cn,direct

# 或使用 Athens
export GOPROXY=https://athens.azurefd.net,direct

# 重新构建
cargo clean
cargo build --release
```

#### 方案 B: 离线构建
```bash
# 在有网络的机器上：
# 1. 下载所有依赖
go mod download

# 2. 打包 $GOPATH/pkg/mod
tar -czf go-deps.tar.gz $GOPATH/pkg/mod

# 在离线机器上：
# 1. 解压依赖
tar -xzf go-deps.tar.gz -C $GOPATH/

# 2. 构建
cargo build --release
```

#### 方案 C: 使用预编译版本
```bash
# 使用无 Go 版本
cargo build --release --no-default-features
```

---

### 问题 2: Go 版本过低

**错误**:
```
requires go >= 1.20
```

**解决**:
```bash
# 下载最新 Go
wget https://go.dev/dl/go1.21.0.linux-amd64.tar.gz

# 安装
sudo tar -C /usr/local -xzf go1.21.0.linux-amd64.tar.gz

# 添加到 PATH
export PATH=$PATH:/usr/local/go/bin

# 验证
go version
```

---

### 问题 3: 构建超时

**错误**:
```
timeout waiting for cargo build
```

**解决**:
```bash
# 增加构建超时时间
export CARGO_BUILD_TIMEOUT=600

# 或使用更少的并行任务
export CARGO_BUILD_JOBS=1

# 重新构建
cargo build --release
```

---

## 🚀 运行模式

### 模式 1: 原生 Tailscale（默认）

```bash
# 使用 libtailscale API
./socktail --authkey "tskey-xxx"

# 输出:
# Using native Tailscale implementation (libtailscale)
# Connecting to Tailscale network via native API...
# ✅ Tailscale loopback: Address: 100.64.x.x, Credential: ...
```

### 模式 2: CLI 模式

```bash
# 构建 CLI 版本
cargo build --release --no-default-features

# 运行
./target/release/socktail --authkey "tskey-xxx"

# 输出:
# Using CLI-based Tailscale implementation
```

### 模式 3: 开发模式（无 VPN）

```bash
# 跳过 VPN 连接
./socktail --no-vpn

# 输出:
# ⚠️  Running in dev mode (no VPN)
# 🚀 Starting SOCKS5 server on 127.0.0.1:1080
```

---

## 📦 发布构建

### 本地发布

```bash
# 构建所有平台（需要 cross）
cargo install cross
cross build --release --target x86_64-unknown-linux-musl

# 或使用脚本
./scripts/build-all.sh
```

### GitHub Actions 自动发布

```bash
# 创建 release tag
git tag v0.1.1
git push origin v0.1.1

# GitHub Actions 会自动：
# 1. 为 5 个平台构建（Linux, macOS, Windows）
# 2. 创建 GitHub Release
# 3. 上传所有二进制文件
```

---

## 🔧 开发建议

### 快速开发迭代

```bash
# 使用 --no-default-features 跳过 libtailscale
cargo build --no-default-features

# 或使用开发模式
cargo run -- --no-vpn
```

### 完整功能测试

```bash
# 完整构建（包含 native Tailscale）
cargo build --release

# 测试
./target/release/socktail --authkey "tskey-xxx"
```

---

## 📚 相关文档

- **libtailscale-rs**: https://github.com/messense/libtailscale-rs
- **libtailscale**: https://github.com/tailscale/libtailscale
- **Tailscale 文档**: https://tailscale.com/kb

---

## ⚙️ 构建配置

### Cargo.toml 配置

```toml
[features]
default = ["native-tailscale"]
native-tailscale = ["libtailscale"]

[dependencies]
libtailscale = { version = "0.2", optional = true }
```

### 环境变量

```bash
# Go 代理
export GOPROXY=https://goproxy.cn,direct

# 构建并行数
export CARGO_BUILD_JOBS=4

# 嵌入 auth key（可选）
export AUTH_KEY="tskey-xxx"
cargo build --release
```

---

## 🎯 总结

**当前方案**: libtailscale-rs
- ✅ 官方维护的 Rust 绑定
- ✅ 功能完整
- ✅ API 简洁
- ⚠️ 需要 Go 1.20+ 编译

**构建流程**:
1. 安装 Go 1.20+
2. `cargo build --release`
3. 首次构建 3-5 分钟（下载并编译 libtailscale）
4. 后续构建 ~30 秒

**无 Go 替代方案**:
- 使用 `--no-default-features` 构建
- 需要系统安装 tailscale CLI
