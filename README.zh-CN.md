# 收纳 (Shouna)

[English](./README.md) · 简体中文

> macOS 菜单栏图标隐藏 — 给被刘海挤掉的图标一个"收纳区"。

姊妹项目：剪切板管理器 [贴贴 (tietie)](https://github.com/wangxiuwen/tietie)。

## ✨ 是什么

带刘海的 MacBook Pro 菜单栏空间紧张，第三方 app 的图标常常被挤到看不见。

**收纳** 用 HiddenBar / Ice / Dozer 的经典方案：在菜单栏放 2 个自家 NSStatusItem：

```
[ 其它 app 图标 …  ] ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ ▸
                    ↑                                  ↑
                  分隔符                         折叠/展开按钮
```

- **展开态**：分隔符宽 1pt，所有图标可见
- **折叠态**：分隔符宽 10000pt → 把它左侧的图标挤出屏幕

**没有私有 API**，没有 Accessibility，对系统零修改。

## 🚀 用法

1. 安装并启动收纳
2. 菜单栏右侧会出现 `▸` 按钮（旁边一个细分隔符）
3. **按住 ⌘ 拖动** 那些"不想常驻"的 app 图标，把它们放到分隔符左侧
4. 点 `▸` 折叠，再点 `▾` 展开 — 一键收纳

也可用全局快捷键 **`⌃⌥H`** 在任何 app 里折叠/展开。

## 📦 下载

[Releases](https://github.com/wangxiuwen/shouna/releases) 提供：

| 架构 | 包 |
|---|---|
| Apple Silicon | `Shouna_x.y.z_aarch64.dmg` |
| Intel | `Shouna_x.y.z_x64.dmg` |

仅支持 macOS 11+（Big Sur 及以上）。

## 🛠 本地开发

```bash
npm install
npm run tauri:dev
```

依赖 `Node 20+`、`Rust stable`、macOS。

## 🗺 路线图

- [x] **v0.1** — 2-NSStatusItem 折叠/展开 + 全局快捷键 + 偏好窗
- [ ] **v0.2** — 3 段式（永远显示 / 可收纳 / 一直隐藏）
- [ ] **v0.3** — 每 app 单独的"始终隐藏"开关（需 Accessibility）
- [ ] **v0.4** — 拖拽时的视觉对齐辅助线

## 🙏 致谢

技术方案参考开源项目：
- [HiddenBar](https://github.com/dwarvesf/hidden) — Swift
- [Ice](https://github.com/jordanbaird/Ice) — Swift
- [Dozer](https://github.com/Mortennn/Dozer) — Swift

收纳是 Tauri 2 + Rust 重新实现，体积小（~6 MB），与上面的 Swift 同行价值定位一致。

## 📜 License

MIT
