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
  const [rightDist, setRightDist] = useState<number | null>(null);

  useEffect(() => {
    invoke<Config>("get_config").then(setConfig);
    invoke<boolean>("has_accessibility").then(setAccessibility);
    const tick = () => invoke<number>("distance_from_right_edge").then(setRightDist);
    tick();
    const t = setInterval(tick, 2000);
    return () => clearInterval(t);
  }, []);

  const update = async (patch: Partial<Config>) => {
    const next = await invoke<Config>("update_config", { patch });
    setConfig(next);
  };

  return (
    <div className="app">
      <div className="titlebar" data-tauri-drag-region />

      <header className="hero">
        <div className="logo">囤</div>
        <div>
          <h1>囤囤</h1>
          <p>把 macOS 菜单栏图标囤起来，按需取用。</p>
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

      {rightDist !== null && rightDist > 150 && (
        <section className="alert">
          <div>
            <strong>把按钮拖到菜单栏最右</strong>
            <p>
              按住 <kbd>⌘</kbd> 把菜单栏里的 <code>{config?.collapsed ? "▾" : "▸"}</code> 按钮拖到最右边（紧挨 Wi-Fi/电池/时钟左侧）。
              macOS 会记住位置，下次启动还在那里。
              <br />
              当前距右边缘约 <b>{Math.round(rightDist)} pt</b>，目标 ≤150 pt。
            </p>
          </div>
        </section>
      )}

      <section className="card">
        <h2>当前状态</h2>
        <p className="hint">
          囤囤在菜单栏放 <b>1 个隐形分隔符</b> + <b>1 个折叠按钮</b> <code>▸</code>。<br />
          用 <kbd>⌘</kbd> 拖动其它 app 的图标到分隔符左侧（"腮帮子"），点 <code>▸</code> 折叠就能把它们囤起来。
        </p>

        <div className="row">
          <div>
            <div className="label">腮帮子状态</div>
            <div className="value">
              {config?.collapsed ? (
                <span className="badge on">已囤起 · 图标藏在腮帮子里</span>
              ) : (
                <span className="badge off">已吐出 · 图标可见</span>
              )}
            </div>
          </div>
          <button className="btn primary" onClick={() => invoke("toggle_collapsed")}>
            {config?.collapsed ? "吐出来" : "囤进去"}
          </button>
        </div>
      </section>

      <section className="card">
        <h2>偏好</h2>

        <div className="row">
          <div>
            <div className="label">显示折叠/展开按钮 (▸)</div>
            <div className="hint">放在菜单栏最右，单击 = 囤进去/吐出来</div>
          </div>
          <Toggle
            value={config?.show_toggle_button ?? true}
            onChange={(v) => update({ show_toggle_button: v })}
          />
        </div>

        <div className="row">
          <div>
            <div className="label">全局快捷键</div>
            <div className="hint">在任意 app 里按 = 囤进去/吐出来</div>
          </div>
          <code className="kbd-display">{config?.hotkey ?? "⌃⌥H"}</code>
        </div>
      </section>

      <section className="card">
        <h2>关于</h2>
        <p className="hint">
          囤囤采用 HiddenBar / Ice / Dozer 的 NSStatusItem 分隔符方案，<b>不使用 macOS 私有 API</b>，对系统无修改。
          要把其它 app 的图标囤起来，需在菜单栏按 <kbd>⌘</kbd> + 拖动那些图标到分隔符左侧 (macOS 自带功能)。
        </p>
        <div className="meta">
          囤囤 v0.1.0 · macOS · Tauri 2 + objc2
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
