mod config;
#[cfg(target_os = "macos")]
mod menubar;

use config::{Config, ConfigPatch};
use std::sync::Arc;
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut builder = tauri::Builder::default().plugin(tauri_plugin_autostart::init(
        tauri_plugin_autostart::MacosLauncher::LaunchAgent,
        None,
    ));

    #[cfg(desktop)]
    {
        builder = builder.plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_from_anywhere(app.clone());
                    }
                })
                .build(),
        );
    }

    builder
        .setup(|app| {
            // config
            let dir = app
                .path()
                .app_local_data_dir()
                .expect("no app local data dir");
            config::init(dir.join("config.json"));
            let cfg = config::get();

            // tray
            #[cfg(desktop)]
            create_tray(app.handle())?;

            // menubar items (macOS)
            #[cfg(target_os = "macos")]
            {
                let app_handle = app.handle().clone();
                let cb = Arc::new(move |collapsed: bool| {
                    let _ = config::apply(ConfigPatch {
                        collapsed: Some(collapsed),
                        ..Default::default()
                    });
                    let _ = app_handle.emit_to(
                        tauri::EventTarget::any(),
                        "collapsed-changed",
                        collapsed,
                    );
                });

                use tauri::ActivationPolicy;
                app.set_activation_policy(ActivationPolicy::Accessory);

                let mtm = unsafe { objc2::MainThreadMarker::new_unchecked() };
                menubar::install(mtm, cfg.show_toggle_button, cfg.collapsed, Some(cb));
            }

            // hotkey (best-effort)
            #[cfg(desktop)]
            register_hotkey(app.handle(), &cfg.hotkey);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            update_config,
            toggle_collapsed,
            has_accessibility,
            open_accessibility_settings,
            quit_app,
        ])
        .on_window_event(|window, event| {
            if matches!(event, tauri::WindowEvent::CloseRequested { .. })
                && window.label() == "main"
            {
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(desktop)]
fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let toggle = MenuItem::with_id(
        app,
        "toggle",
        "折叠/展开 (囤进腮帮子)",
        true,
        Some("CmdOrCtrl+Alt+H"),
    )?;
    let prefs = MenuItem::with_id(app, "prefs", "偏好设置 …", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let about = MenuItem::with_id(app, "about", "关于囤囤", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&toggle, &prefs, &sep, &about, &quit])?;

    let _tray = TrayIconBuilder::with_id("main")
        .icon(tray_icon_image())
        .icon_as_template(true)
        .tooltip("囤囤 — 菜单栏图标囤起来")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "toggle" => toggle_from_anywhere(app.clone()),
            "prefs" => show_main_window(app),
            "about" => show_main_window(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .build(app)?;
    Ok(())
}

fn tray_icon_image() -> Image<'static> {
    // 16x16 monochrome icon: 3 dots clustered (representing collapsed status items)
    const W: u32 = 16;
    const H: u32 = 16;
    let mut buf = vec![0u8; (W * H * 4) as usize];
    let on = [255, 255, 255, 255];
    let put = |buf: &mut [u8], x: u32, y: u32, c: [u8; 4]| {
        if x < W && y < H {
            let i = ((y * W + x) * 4) as usize;
            buf[i..i + 4].copy_from_slice(&c);
        }
    };
    // a tray-glyph: 3 horizontal bars (like a "stash" icon)
    for x in 4..12 {
        for dy in [4u32, 7, 10] {
            put(&mut buf, x, dy, on);
            put(&mut buf, x, dy + 1, on);
        }
    }
    Image::new_owned(buf, W, H)
}

fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.set_focus();
        #[cfg(target_os = "macos")]
        {
            use tauri::ActivationPolicy;
            let _ = app.set_activation_policy(ActivationPolicy::Regular);
        }
    }
}

#[cfg(desktop)]
fn register_hotkey<R: Runtime>(app: &AppHandle<R>, hotkey: &str) {
    use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
    // Default: Ctrl+Alt+H. Parse simple human form if needed.
    let shortcut = parse_hotkey(hotkey)
        .unwrap_or_else(|| Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyH));
    if let Err(e) = app.global_shortcut().register(shortcut) {
        log::warn!("register hotkey failed: {e}");
    }
}

#[cfg(desktop)]
fn parse_hotkey(s: &str) -> Option<tauri_plugin_global_shortcut::Shortcut> {
    use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
    let mut mods = Modifiers::empty();
    let mut code: Option<Code> = None;
    for ch in s.chars() {
        match ch {
            '⌘' => mods |= Modifiers::SUPER,
            '⇧' => mods |= Modifiers::SHIFT,
            '⌃' => mods |= Modifiers::CONTROL,
            '⌥' => mods |= Modifiers::ALT,
            c if c.is_ascii_alphabetic() => {
                code = Some(match c.to_ascii_uppercase() {
                    'A' => Code::KeyA,
                    'B' => Code::KeyB,
                    'C' => Code::KeyC,
                    'D' => Code::KeyD,
                    'E' => Code::KeyE,
                    'F' => Code::KeyF,
                    'G' => Code::KeyG,
                    'H' => Code::KeyH,
                    'I' => Code::KeyI,
                    'J' => Code::KeyJ,
                    'K' => Code::KeyK,
                    'L' => Code::KeyL,
                    'M' => Code::KeyM,
                    'N' => Code::KeyN,
                    'O' => Code::KeyO,
                    'P' => Code::KeyP,
                    'Q' => Code::KeyQ,
                    'R' => Code::KeyR,
                    'S' => Code::KeyS,
                    'T' => Code::KeyT,
                    'U' => Code::KeyU,
                    'V' => Code::KeyV,
                    'W' => Code::KeyW,
                    'X' => Code::KeyX,
                    'Y' => Code::KeyY,
                    'Z' => Code::KeyZ,
                    _ => return None,
                });
            }
            _ => {}
        }
    }
    code.map(|c| Shortcut::new(Some(mods), c))
}

fn toggle_from_anywhere<R: Runtime>(app: AppHandle<R>) {
    #[cfg(target_os = "macos")]
    {
        let _ = app.run_on_main_thread(move || {
            let mtm = unsafe { objc2::MainThreadMarker::new_unchecked() };
            let now = !menubar::is_collapsed();
            menubar::apply_collapsed(mtm, now);
        });
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
    }
}

// ─────────────────────────── IPC commands ───────────────────────────

#[tauri::command]
fn get_config() -> Config {
    config::get()
}

#[tauri::command]
fn update_config(app: AppHandle, patch: ConfigPatch) -> Config {
    let new_cfg = config::apply(patch);

    #[cfg(target_os = "macos")]
    {
        let new = new_cfg.clone();
        let _ = app.run_on_main_thread(move || {
            let mtm = unsafe { objc2::MainThreadMarker::new_unchecked() };
            menubar::set_toggle_visible(mtm, new.show_toggle_button);
            menubar::apply_collapsed(mtm, new.collapsed);
        });
    }

    new_cfg
}

#[tauri::command]
fn toggle_collapsed(app: AppHandle) {
    toggle_from_anywhere(app);
}

#[tauri::command]
fn has_accessibility() -> bool {
    // 囤囤 MVP 用公开 API, 不强需要辅助功能。返回 true 简化 UI。
    // Phase 2 接入 AX 后改为真实检测。
    true
}

#[tauri::command]
fn open_accessibility_settings() {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .status();
    }
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[allow(unused_imports)]
use tauri::Emitter;
