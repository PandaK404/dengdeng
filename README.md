# 续言

> **让 AI 对话自然延续，把关键问题聊透**

如果你经常遇到 AI 助手在任务尚未澄清、方案尚未确认、细节尚未落地时就准备结束对话，`续言` 的作用就是把这段交互接住。

当 AI 试图提前收尾时，`续言` 会及时弹出交互窗口，让你补充信息、选择方向或要求继续推进，直到问题真正解决。

## 🌟 核心特性

- 🛑 **智能拦截**：AI 想结束时自动弹出继续选项
- 🧠 **记忆管理**：按项目存储开发规范和偏好
- 🎨 **优雅交互**：Markdown 支持、多种输入方式
- ⚡ **即装即用**：3 秒安装，跨平台支持

## 📸 看看效果

### 🛑 智能拦截弹窗
![续言弹窗演示](./screenshots/popup.png)

*当 AI 试图结束对话时，续言会及时弹出交互窗口，提供预定义选项和补充输入能力，让交流继续推进*

### ⚙️ 设置管理界面
![续言设置界面](./screenshots/settings.png)

*优雅的设置界面，支持记忆管理、功能开关、主题切换和智能提示词生成*

## 🚀 开始使用

### 方式一：手动下载（推荐）

1. 访问 [Releases 页面](https://github.com/PandaK404/dengdeng/releases)
2. 下载适合你系统的版本：
   - 🐧 **Linux**: `xuyan-cli-v*-linux-x86_64.tar.gz`
   - 🍎 **macOS (Intel)**: `xuyan-cli-v*-macos-x86_64.tar.gz`
   - 🍎 **macOS (Apple Silicon)**: `xuyan-cli-v*-macos-aarch64.tar.gz`
   - 🪟 **Windows**: `xuyan-cli-v*-windows-x86_64.zip`

3. 解压后将 `续言` 和 `续言设置` 添加到系统 PATH

### 方式二：源码构建

```bash
git clone https://github.com/PandaK404/dengdeng.git
cd dengdeng
pnpm install
pnpm tauri:build
```

> 如果你准备继续维护 Homebrew 分发，请先同步建立新的 tap 和 formula；当前仓库文档不再假定沿用上游的 Homebrew 发布链路。

## ⚡ 快速上手

### 第一步：配置 MCP 客户端

在你的 MCP 客户端（如 Claude Desktop）配置文件中添加：

```json
{
    "mcpServers": {
    "续言": {
      "command": "续言"
    }
  }
}
```

### 第二步：打开设置界面

```bash
# 运行设置命令
续言设置
```

### 第三步：配置提示词

在设置界面的"参考提示词"标签页：
1. 查看自动生成的提示词
2. 点击复制按钮
3. 将提示词添加到你的 AI 助手中

### 第四步：开始使用

现在你的 AI 助手就拥有了智能拦截、记忆管理和弹窗交互功能！

> 💡 **小贴士**：你可以参考生成的提示词进行个性化修改，打造专属的 AI 交互体验。

## 🔧 工具说明

续言提供了多个 MCP 工具来增强 AI 助手的能力：

- **代码搜索工具**：基于 ACE 的语义代码搜索，帮助 AI 理解项目代码结构
  - 📖 [详细使用说明](./ACEMCP.md)

### 🙏 致谢

感谢以下开源项目及其贡献者：

- **[acemcp](https://github.com/qy527145/acemcp)** - 由 [@qy527145](https://github.com/qy527145) 开发，提供了强大的代码库语义搜索能力。本项目在保留原有功能的基础上，使用 Rust 重写了核心逻辑并集成到续言的 MCP 工具生态中。

## 🤝 参与贡献

续言是开源项目，我们欢迎所有形式的贡献！

### 🛠️ 本地开发
```bash
git clone https://github.com/PandaK404/dengdeng.git
cd dengdeng
pnpm install
pnpm tauri:dev
```

## 📄 开源协议

MIT License - 自由使用，欢迎贡献！
