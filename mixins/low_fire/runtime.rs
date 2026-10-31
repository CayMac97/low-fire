// Low Fire - Runtime Module
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub const ID: &str = "low_fire";
pub const NAME: &str = "Low Fire";
pub const VERSION: &str = "1.0.1";
pub const DESCRIPTION: &str =
    "Lowers or hides the first-person burning fire overlay for crystal clear vision in PvP and survival.";

pub const KEY_TOGGLE: &str = "toggle";
pub const KEY_TOGGLE_LABEL: &str = "Toggle Low Fire";
pub const KEY_TOGGLE_DEFAULT: &str = "key.keyboard.unknown";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowFireMode {
    Low,       // 22% height
    VeryLow,   // 14% height
    Minimal,   // 8% height
    NoFire,    // 0% (hidden)
    Vanilla,   // 100% normal
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowFireOpacity {
    Full,         // 100%
    Subtle,       // 70%
    Translucent,  // 45%
}

static ENABLED: AtomicBool = AtomicBool::new(true);
static MODE: AtomicU8 = AtomicU8::new(0); // 0=Low, 1=VeryLow, 2=Minimal, 3=NoFire, 4=Vanilla
static OPACITY: AtomicU8 = AtomicU8::new(0); // 0=Full, 1=Subtle, 2=Translucent

#[inline]
pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn toggle_enabled() -> bool {
    let prev = ENABLED.fetch_xor(true, Ordering::Relaxed);
    !prev
}

#[inline]
pub fn mode() -> LowFireMode {
    match MODE.load(Ordering::Relaxed) {
        0 => LowFireMode::Low,
        1 => LowFireMode::VeryLow,
        2 => LowFireMode::Minimal,
        3 => LowFireMode::NoFire,
        _ => LowFireMode::Vanilla,
    }
}

pub fn cycle_mode() -> LowFireMode {
    let cur = MODE.load(Ordering::Relaxed);
    let next = (cur + 1) % 5;
    MODE.store(next, Ordering::Relaxed);
    mode()
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

#[inline]
pub fn opacity() -> LowFireOpacity {
    match OPACITY.load(Ordering::Relaxed) {
        0 => LowFireOpacity::Full,
        1 => LowFireOpacity::Subtle,
        _ => LowFireOpacity::Translucent,
    }
}

pub fn cycle_opacity() -> LowFireOpacity {
    let cur = OPACITY.load(Ordering::Relaxed);
    let next = (cur + 1) % 3;
    OPACITY.store(next, Ordering::Relaxed);
    opacity()
}

pub fn opacity_label() -> &'static str {
    match opacity() {
        LowFireOpacity::Full => "100%",
        LowFireOpacity::Subtle => "70%",
        LowFireOpacity::Translucent => "45%",
    }
}

#[inline]
pub fn fire_y_offset() -> f32 {
    match mode() {
        LowFireMode::Low => 0.40,
        LowFireMode::VeryLow => 0.54,
        LowFireMode::Minimal => 0.67,
        LowFireMode::NoFire => 2.0,
        LowFireMode::Vanilla => 0.0,
    }
}

#[inline]
pub fn fire_opacity_factor() -> f32 {
    match opacity() {
        LowFireOpacity::Full => 1.0,
        LowFireOpacity::Subtle => 0.70,
        LowFireOpacity::Translucent => 0.45,
    }
}
