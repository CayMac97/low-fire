// Low Fire runtime module
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub const ID: &str = "low_fire";
pub const NAME: &str = "Low Fire";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Lowers or hides the first-person burning fire overlay for crystal clear vision in PvP and survival.";

pub const KEY_TOGGLE: &str = "toggle";
pub const KEY_TOGGLE_LABEL: &str = "Toggle Low Fire";
pub const KEY_TOGGLE_DEFAULT: &str = "key.keyboard.unknown";

static ENABLED: AtomicBool = AtomicBool::new(true);
static MODE: AtomicU8 = AtomicU8::new(0); // 0 = Low, 1 = Very Low, 2 = Minimal, 3 = No Fire, 4 = Vanilla
static OPACITY: AtomicU8 = AtomicU8::new(0); // 0 = 100%, 1 = 75%, 2 = 50%

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LowFireMode {
    Low,       // 22% height
    VeryLow,   // 14% height
    Minimal,   // 8% height
    NoFire,    // 0% (hidden completely)
    Vanilla,   // 100%
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LowFireOpacity {
    Full,
    Subtle,
    Translucent,
}

pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn toggle_enabled() -> bool {
    let prev = ENABLED.fetch_xor(true, Ordering::Relaxed);
    !prev
}

pub fn mode() -> LowFireMode {
    match MODE.load(Ordering::Relaxed) % 5 {
        0 => LowFireMode::Low,
        1 => LowFireMode::VeryLow,
        2 => LowFireMode::Minimal,
        3 => LowFireMode::NoFire,
        _ => LowFireMode::Vanilla,
    }
}

pub fn cycle_mode() {
    let current = MODE.load(Ordering::Relaxed);
    MODE.store((current + 1) % 5, Ordering::Relaxed);
}

pub fn mode_label() -> &'static str {
    match mode() {
        LowFireMode::Low => "Low (22%)",
        LowFireMode::VeryLow => "Very Low (14%)",
        LowFireMode::Minimal => "Minimal (8%)",
        LowFireMode::NoFire => "No Fire (Hidden)",
        LowFireMode::Vanilla => "Vanilla (100%)",
    }
}

pub fn opacity() -> LowFireOpacity {
    match OPACITY.load(Ordering::Relaxed) % 3 {
        0 => LowFireOpacity::Full,
        1 => LowFireOpacity::Subtle,
        _ => LowFireOpacity::Translucent,
    }
}

pub fn cycle_opacity() {
    let current = OPACITY.load(Ordering::Relaxed);
    OPACITY.store((current + 1) % 3, Ordering::Relaxed);
}

pub fn opacity_label() -> &'static str {
    match opacity() {
        LowFireOpacity::Full => "100%",
        LowFireOpacity::Subtle => "75%",
        LowFireOpacity::Translucent => "50%",
    }
}
