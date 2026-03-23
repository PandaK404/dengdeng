# 续言 安装指南

## 快速安装

### 方式一：下载预编译版本

从 [Releases](https://github.com/PandaK404/dengdeng/releases) 页面下载对应平台的预编译版本：

- **Linux**: `xuyan-cli-v*-linux-x86_64.tar.gz`
- **macOS (Intel)**: `xuyan-cli-v*-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `xuyan-cli-v*-macos-aarch64.tar.gz`
- **Windows**: `xuyan-cli-v*-windows-x86_64.zip`

安装步骤：

1. 下载对应平台的压缩包
2. 解压到任意目录
3. 将解压后的 `续言` 和 `续言设置` 添加到 `PATH`

Linux/macOS 示例：
```bash
tar -xzf xuyan-cli-v0.4.0-linux-x86_64.tar.gz
sudo cp 续言 续言设置 /usr/local/bin/
```

### 方式二：从源码构建

```bash
git clone https://github.com/PandaK404/dengdeng.git
cd dengdeng
pnpm install
pnpm build
cargo build --release
```

构建完成后，二进制位于：

- `target/release/续言设置`
- `target/release/续言`

可按需安装到本地：

```bash
cp target/release/续言 target/release/续言设置 ~/.local/bin/
```

## 验证安装

```bash
续言 --help
续言设置 --help
```

## MCP 客户端配置

将以下配置添加到 MCP 客户端配置文件：

```json
{
  "mcpServers": {
    "续言": {
      "command": "续言"
    }
  }
}
```

## 使用方法

### MCP 服务器模式

```bash
续言
```

### 弹窗界面模式

```bash
续言设置
续言设置 --mcp-request file
```

## 工具说明

- `续言`: MCP 服务器，提供交互、记忆和代码搜索能力
- `续言设置`: 弹窗界面与设置界面

## 发布说明

- 如果你继续维护自动更新、Homebrew 或发布资产，请确保 release 产物前缀与仓库配置保持一致：`xuyan-cli-*`。

## 系统要求

- **Linux**: x86_64
- **macOS**: 10.15+
- **Windows**: Windows 10+ x86_64
