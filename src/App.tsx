import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Config {
  collapsed: boolean;
  show_toggle_button: boolean;
  hotkey: string;
}

export default function App() {
  const [config, setConfig] = useState<Config | null>(null);
  const [accessibility, setAccessibility] = useState<boolean | null>(null);

  useEffect(() => {
    invoke<Config>("get_config").then(setConfig);
    invoke<boolean>("has_accessibility").then(setAccessibility);
  }, []);

  const update = async (patch: Partial<Config>) => {
    const next = await invoke<Config>("update_config", { patch });
    setConfig(next);
  };

  return (
    <div className="app">
      <div className="titlebar" data-tauri-drag-region />

      <header className="hero">
        <div className="logo">收</div>
        <div>
          <h1>收纳</h1>
          <p>整理 macOS 菜单栏图标 — 隐藏不常用的，按需展开。</p>
        </div>
      </header>

      {accessibility === false && (
        <section className="alert">
          <div>
            <strong>需要辅助功能权限</strong>
            <p>用于读取与移动其它 app 的菜单栏图标位置。</p>
          </div>
          <button className="btn primary" onClick={() => invoke("open_accessibility_settings")}>
            打开系统设置
          </button>
        </section>
      )}

      <section className="card">
        <h2>当前状态</h2>
        <p className="hint">
          收纳在菜单栏放 <b>3 个分隔符</b>: <code>•</code> <code>━</code> <code>▸</code>。<br />
          用 <kbd>⌘</kbd> 拖动其它 app 的图标，把它们放到 <code>•</code> 和 <code>━</code> 之间（"收纳区"），点 <code>▸</code> 即可一键隐藏/展开。
        </p>

        <div className="row">
          <div>
            <div className="label">收纳区状态</div>
            <div className="value">
              {config?.collapsed ? (
                <span className="badge on">已折叠 · 图标隐藏</span>
              ) : (
                <span className="badge off">已展开 · 图标可见</span>
              )}
            </div>
          </div>
          <button className="btn primary" onClick={() => invoke("toggle_collapsed")}>
            {config?.collapsed ? "展开" : "折叠"}
          </button>
        </div>
      </section>

      <section className="card">
        <h2>偏好</h2>

        <div className="row">
          <div>
            <div className="label">显示折叠/展开按钮 (▸)</div>
            <div className="hint">放在菜单栏最右，单击切换收纳区</div>
          </div>
          <Toggle
            value={config?.show_toggle_button ?? true}
            onChange={(v) => update({ show_toggle_button: v })}
          />
        </div>

        <div className="row">
          <div>
            <div className="label">全局快捷键</div>
            <div className="hint">在任意 app 里按 = 折叠/展开收纳区</div>
          </div>
          <code className="kbd-display">{config?.hotkey ?? "⌃⌥H"}</code>
        </div>
      </section>

      <section className="card">
        <h2>关于</h2>
        <p className="hint">
          收纳采用 HiddenBar / Ice / Dozer 的"3 个 NSStatusItem 分隔符"方案，<b>不使用 macOS 私有 API</b>，对系统无修改。
          要把其它 app 的图标放到"收纳区"，需在菜单栏按 <kbd>⌘</kbd> + 拖动那些图标 (macOS 自带功能)。
        </p>
        <div className="meta">
          收纳 v0.1.0 · macOS · Tauri 2 + objc2
        </div>
      </section>
    </div>
  );
}

function Toggle({ value, onChange }: { value: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      className={"toggle" + (value ? " on" : "")}
      onClick={() => onChange(!value)}
      aria-label="toggle"
    />
  );
}
