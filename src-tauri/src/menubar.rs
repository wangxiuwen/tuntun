//! macOS NSStatusItem manipulation — HiddenBar/Ice/Dozer 风格的 3 项分隔方案。
//!
//! 思路:
//!   - 注册 2 个自家 NSStatusItem: SEPARATOR (隐形扩展条) + TOGGLE (按钮)
//!   - 折叠态: SEPARATOR.length = HUGE -> 把它左侧的其它 app 图标挤出屏幕
//!   - 展开态: SEPARATOR.length = 1 -> 让出空间
//!   - 用户用 ⌘ 拖动其它 app 图标到 SEPARATOR 左侧, 即"囤进腮帮子"
//!
//! 全部使用 macOS 公开 API, 无私有调用。

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{define_class, msg_send, sel, AllocAnyThread, MainThreadMarker};
use objc2_app_kit::{NSStatusBar, NSStatusItem};
use objc2_foundation::{NSObject, NSString};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;

const LEN_EXPANDED: f64 = 1.0;
const LEN_COLLAPSED: f64 = 10000.0;

const ICON_COLLAPSED: &str = "▸";
const ICON_EXPANDED: &str = "▾";

/// Global handle to the menubar state. Only touched from the main thread.
static STATE: OnceCell<MainThreadGuarded> = OnceCell::new();

struct MainThreadGuarded(Mutex<Option<MenuBarState>>);

/// SAFETY: callers must only acquire the inner Mutex on the main thread.
unsafe impl Send for MainThreadGuarded {}
unsafe impl Sync for MainThreadGuarded {}

struct MenuBarState {
    separator: Retained<NSStatusItem>,
    toggle: Retained<NSStatusItem>,
    _target: Retained<ToggleTarget>,
    collapsed: bool,
    on_change: Option<Arc<dyn Fn(bool) + Send + Sync + 'static>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "SHToggleTarget"]
    pub struct ToggleTarget;

    impl ToggleTarget {
        #[unsafe(method(onClick:))]
        fn on_click(&self, _sender: *mut AnyObject) {
            // SAFETY: NSStatusBarButton action callbacks run on the main thread.
            let mtm = unsafe { MainThreadMarker::new_unchecked() };
            let current = is_collapsed();
            apply_collapsed(mtm, !current);
        }
    }
);

/// Initialize the menubar items. Must be called from the main thread.
pub fn install(
    mtm: MainThreadMarker,
    show_toggle_button: bool,
    initial_collapsed: bool,
    on_change: Option<Arc<dyn Fn(bool) + Send + Sync + 'static>>,
) {
    unsafe {
        let bar = NSStatusBar::systemStatusBar();

        let separator = bar.statusItemWithLength(LEN_EXPANDED);
        // autosaveName persists user's ⌘+drag position across launches (macOS 10.12+)
        let _: () = msg_send![
            &*separator,
            setAutosaveName: &*NSString::from_str("com.tuntun.menubar.separator")
        ];
        if let Some(btn) = separator.button(mtm) {
            btn.setTitle(&NSString::from_str(""));
        }

        let toggle_len = if show_toggle_button { 28.0 } else { 0.0 };
        let toggle = bar.statusItemWithLength(toggle_len);
        let _: () = msg_send![
            &*toggle,
            setAutosaveName: &*NSString::from_str("com.tuntun.menubar.toggle")
        ];

        let target: Retained<ToggleTarget> = msg_send![ToggleTarget::alloc(), init];
        if let Some(btn) = toggle.button(mtm) {
            btn.setTitle(&NSString::from_str(ICON_COLLAPSED));
            let _: () = msg_send![&*btn, setTarget: &*target];
            let _: () = msg_send![&*btn, setAction: sel!(onClick:)];
        }

        let state = MenuBarState {
            separator,
            toggle,
            _target: target,
            collapsed: false,
            on_change,
        };

        STATE
            .set(MainThreadGuarded(Mutex::new(Some(state))))
            .map_err(|_| "already installed")
            .ok();

        if initial_collapsed {
            apply_collapsed(mtm, true);
        }
    }
}

pub fn is_collapsed() -> bool {
    STATE
        .get()
        .and_then(|g| g.0.lock().as_ref().map(|s| s.collapsed))
        .unwrap_or(false)
}

/// Mutate UI on the main thread. Caller provides the marker.
pub fn apply_collapsed(mtm: MainThreadMarker, collapsed: bool) {
    let Some(guarded) = STATE.get() else { return };
    let mut slot = guarded.0.lock();
    let Some(state) = slot.as_mut() else { return };

    state.collapsed = collapsed;
    state.separator.setLength(if collapsed {
        LEN_COLLAPSED
    } else {
        LEN_EXPANDED
    });
    if let Some(btn) = state.toggle.button(mtm) {
        btn.setTitle(&NSString::from_str(if collapsed {
            ICON_EXPANDED
        } else {
            ICON_COLLAPSED
        }));
    }
    if let Some(cb) = state.on_change.clone() {
        drop(slot);
        cb(collapsed);
    }
}

pub fn set_toggle_visible(mtm: MainThreadMarker, visible: bool) {
    let Some(guarded) = STATE.get() else { return };
    let slot = guarded.0.lock();
    let Some(state) = slot.as_ref() else { return };
    state.toggle.setLength(if visible { 28.0 } else { 0.0 });
    let _ = mtm;
}

/// Distance of the toggle button from the right edge of the main screen, in points.
/// Returns None if not installed yet or button has no window.
pub fn distance_from_right_edge(mtm: MainThreadMarker) -> Option<f64> {
    use objc2_app_kit::NSScreen;
    let guarded = STATE.get()?;
    let slot = guarded.0.lock();
    let state = slot.as_ref()?;
    let btn = state.toggle.button(mtm)?;
    let window = btn.window()?;
    let frame = window.frame();
    let screen = NSScreen::mainScreen(mtm)?;
    let screen_frame = screen.frame();
    let right_edge = screen_frame.origin.x + screen_frame.size.width;
    Some(right_edge - (frame.origin.x + frame.size.width))
}
