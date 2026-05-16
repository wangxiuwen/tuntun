use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub collapsed: bool,
    pub show_toggle_button: bool,
    pub hotkey: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            collapsed: false,
            show_toggle_button: true,
            hotkey: "⌃⌥H".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigPatch {
    pub collapsed: Option<bool>,
    pub show_toggle_button: Option<bool>,
    pub hotkey: Option<String>,
}

static STORE: OnceLock<Mutex<Store>> = OnceLock::new();

struct Store {
    path: PathBuf,
    cfg: Config,
}

pub fn init(path: PathBuf) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let cfg = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<Config>(&s).ok())
        .unwrap_or_default();
    let _ = STORE.set(Mutex::new(Store { path, cfg }));
}

pub fn get() -> Config {
    STORE
        .get()
        .map(|s| s.lock().cfg.clone())
        .unwrap_or_default()
}

pub fn apply(patch: ConfigPatch) -> Config {
    if let Some(store) = STORE.get() {
        let mut s = store.lock();
        if let Some(v) = patch.collapsed {
            s.cfg.collapsed = v;
        }
        if let Some(v) = patch.show_toggle_button {
            s.cfg.show_toggle_button = v;
        }
        if let Some(v) = patch.hotkey {
            s.cfg.hotkey = v;
        }
        let _ = persist(&s);
        return s.cfg.clone();
    }
    Config::default()
}

fn persist(s: &Store) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&s.cfg).unwrap_or_default();
    std::fs::write(&s.path, json)
}
