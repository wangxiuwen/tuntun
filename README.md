# Shouna (收纳)

English · [简体中文](./README.zh-CN.md)

> macOS menu-bar icon hiding — a "stash" zone for icons the notch eats.

Sister project: clipboard manager [Tietie (贴贴)](https://github.com/wangxiuwen/tietie).

## ✨ What it does

On notched MacBook Pros the menu bar is cramped; third-party app icons often get pushed off-screen.

**Shouna** uses the classic HiddenBar / Ice / Dozer technique: it adds two of its own `NSStatusItem`s to the menu bar:

```
[ other apps … ]  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ▸
                  ↑                                 ↑
                 separator                  toggle button
```

- **Expanded**: separator is 1 pt wide, all icons visible
- **Collapsed**: separator stretches to 10000 pt, pushing the icons on its left off-screen

**No private APIs**, no Accessibility, zero system modifications.

## 🚀 How to use

1. Install and launch Shouna
2. A `▸` button appears on the right of the menu bar (with a thin separator next to it)
3. **Hold ⌘ and drag** the icons you want to stash so they sit to the *left* of the separator
4. Click `▸` to collapse, click `▾` to expand — one-click stash

Or press **`⌃⌥H`** anywhere as a global hotkey to toggle.

## 📦 Download

[Releases](https://github.com/wangxiuwen/shouna/releases) ships:

| Arch | Package |
|---|---|
| Apple Silicon | `Shouna_x.y.z_aarch64.dmg` |
| Intel | `Shouna_x.y.z_x64.dmg` |

macOS 11+ (Big Sur and up) only.

> The app is unsigned. On first launch open *System Settings → Privacy & Security* and click *Open Anyway*.

## 🛠 Local development

```bash
npm install
npm run tauri:dev
```

Requires `Node 20+`, `Rust stable`, macOS.

## 🗺 Roadmap

- [x] **v0.1** — 2-NSStatusItem collapse/expand + global hotkey + preferences window
- [ ] **v0.2** — 3-section layout (always-visible / stashable / always-hidden)
- [ ] **v0.3** — per-app "always hide" toggle (needs Accessibility)
- [ ] **v0.4** — visual alignment guides while dragging

## 🙏 Credits

Approach informed by these open-source projects (all Swift):

- [HiddenBar](https://github.com/dwarvesf/hidden)
- [Ice](https://github.com/jordanbaird/Ice)
- [Dozer](https://github.com/Mortennn/Dozer)

Shouna re-implements the idea in Tauri 2 + Rust, smaller binary (~6 MB), same value proposition.

## 📜 License

MIT
