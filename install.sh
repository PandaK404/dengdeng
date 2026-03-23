#!/bin/bash

# 续言 MCP 工具 - 最简化安装脚本
# 只需构建两个CLI工具即可运行MCP

set -e

echo "🚀 安装 续言 MCP 工具..."

# 检查必要工具
for cmd in cargo pnpm; do
    if ! command -v "$cmd" &> /dev/null; then
        echo "❌ 请先安装 $cmd"
        exit 1
    fi
done

# 构建
echo "📦 构建前端资源..."
pnpm build

echo "🔨 构建 CLI 工具..."
cargo build --release

# 检查构建结果
if [[ ! -f "target/release/续言设置" ]] || [[ ! -f "target/release/续言" ]]; then
    echo "❌ 构建失败"
    exit 1
fi

# 安装到用户目录
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

cp "target/release/续言设置" "$BIN_DIR/"
cp "target/release/续言" "$BIN_DIR/"
chmod +x "$BIN_DIR/续言设置" "$BIN_DIR/续言"

echo "✅ 安装完成！CLI 工具已安装到 $BIN_DIR"

# 检查PATH
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo ""
    echo "💡 请将以下内容添加到 ~/.bashrc 或 ~/.zshrc:"
    echo "export PATH=\"\$PATH:$BIN_DIR\""
    echo "然后运行: source ~/.bashrc"
fi

echo ""
echo "📋 使用方法："
echo "  续言        - 启动 MCP 服务器"
echo "  续言设置      - 启动弹窗界面"
echo ""
echo "📝 MCP 客户端配置："
echo '{"mcpServers": {"续言": {"command": "续言"}}}'
