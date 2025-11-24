# libtailscale 预编译库目录

## 📦 放置文件

将您编译好的 libtailscale 文件放在这里：

```
lib/
├── libtailscale.a    ← 静态库文件
└── libtailscale.h    ← 头文件（可选）
```

## 使用方法

### 1. 复制文件

```bash
# 假设您的文件在当前目录
cp libtailscale.a lib/
cp libtailscale.h lib/  # 可选
```

### 2. 构建项目

```bash
# 构建时会自动检测 lib/libtailscale.a
cargo build --release

# 输出会显示：
# warning: Found pre-compiled libtailscale.a in lib/ directory
```

### 3. 运行

```bash
./target/release/socktail --authkey "tskey-xxx"
```

## ✅ 优势

- **无需 Go**: 不需要安装 Go 编译器
- **快速构建**: 跳过 libtailscale 编译（节省 3-5 分钟）
- **离线友好**: 不需要网络下载
- **跨平台**: 不同平台使用对应的 .a 文件

## 📝 不同平台的文件

如果需要跨平台编译，准备对应平台的库：

```
lib/
├── linux-x86_64/
│   └── libtailscale.a
├── linux-aarch64/
│   └── libtailscale.a
├── macos-x86_64/
│   └── libtailscale.a
├── macos-aarch64/
│   └── libtailscale.a
└── windows-x86_64/
    └── libtailscale.lib
```

## 🔍 验证

检查库是否被正确识别：

```bash
cargo clean
cargo build --release 2>&1 | grep libtailscale

# 应该看到：
# warning: Found pre-compiled libtailscale.a in lib/ directory
```

## ⚠️ 注意事项

1. **平台匹配**: 确保 .a 文件是为当前平台编译的
2. **架构匹配**: x86_64 和 ARM64 不能混用
3. **版本兼容**: 使用最新版本的 libtailscale

## 🆘 故障排查

### 构建失败

**错误**: `unable to find library -ltailscale`

**解决**:
```bash
# 确认文件存在
ls -lh lib/libtailscale.a

# 如果不存在，放置文件
cp /path/to/libtailscale.a lib/
```

### 运行时错误

**错误**: `Failed to create Tailscale instance`

**解决**:
- 检查库是否为正确平台编译
- 确认 libtailscale 版本兼容
- 尝试重新编译 libtailscale

## 📚 获取 libtailscale

如果您还没有编译好的文件：

### 方法 1: 从源码编译

```bash
git clone https://github.com/tailscale/libtailscale.git
cd libtailscale
make archive
# 生成 libtailscale.a
```

### 方法 2: 从发行版下载

查看 GitHub Releases:
https://github.com/tailscale/libtailscale/releases

### 方法 3: 使用 CLI 模式

如果无法获取预编译库：

```bash
# 构建不带 native-tailscale 的版本
cargo build --release --no-default-features

# 使用系统的 tailscale 命令
./target/release/socktail --authkey "tskey-xxx"
```

---

**完全不需要 Go！只需要预编译的 .a 文件！** 🎉
