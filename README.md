# Tuntun (囤囤)

English · [简体中文](./README.zh-CN.md)

> Stash macOS menu-bar icons like a hamster stuffing food in its cheeks.

Sister project: clipboard manager [Tietie (贴贴)](https://github.com/wangxiuwen/tietie).

## What it does

On notched MacBook Pros the menu bar is cramped; third-party app icons often get pushed off-screen.

**Tuntun** uses the classic HiddenBar / Ice / Dozer technique: it adds two of its own `NSStatusItem`s to the menu bar:

```
[ other apps ... ]  --------------------------------  >
                    ^                                 ^
                 separator                  stash/spit toggle
```

- **Spit out** (expanded): separator is 1 pt wide, all icons visible
- **Stash** (collapsed): separator stretches to 10000 pt, pushing the icons on its left off-screen

No private APIs, no Accessibility, zero system modifications.

## How to use

1. Install and launch Tuntun
2. A `>` button shows up on the right of the menu bar (with a thin separator next to it)
3. Hold ⌘ and drag the icons you want to stash so they sit to the *left* of the separator
4. Click `>` to stash, click again to spit them back

Or press **`⌃⌥H`** anywhere as a global hotkey to toggle.

## Download

[Releases](https://github.com/wangxiuwen/tuntun/releases):

| Arch | Package |
|---|---|
| Apple Silicon | `Tuntun_x.y.z_aarch64.dmg` |
| Intel | `Tuntun_x.y.z_x64.dmg` |

macOS 11+ (Big Sur and up) only.

> The app is unsigned. On first launch open *System Settings > Privacy & Security* and click *Open Anyway*.

## Local development

```bash
npm install
npm run tauri:dev
```

Requires `Node 20+`, `Rust stable`, macOS.

## Roadmap

- [x] **v0.1** — 2-NSStatusItem stash/spit + global hotkey + preferences window
- [ ] **v0.2** — 3-section layout (always-visible / stashable / always-hidden)
- [ ] **v0.3** — per-app "always stash" toggle (needs Accessibility)
- [ ] **v0.4** — visual alignment guides while dragging

## Credits

Approach informed by these open-source projects (all Swift):

- [HiddenBar](https://github.com/dwarvesf/hidden)
- [Ice](https://github.com/jordanbaird/Ice)
- [Dozer](https://github.com/Mortennn/Dozer)

Tuntun re-implements the idea in Tauri 2 + Rust, smaller binary (~6 MB).

## License

MIT
