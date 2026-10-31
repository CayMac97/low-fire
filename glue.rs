// Low Fire - Glue Module
use crate::game::Game;
use crate::game::native_hooks::NativeMod;
use horizon_client_render::mod_low_fire as m;

pub(super) fn on_key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_on(m::ID) {
        return;
    }
    let toggle_key = game.native_mod_key(m::ID, m::KEY_TOGGLE, m::KEY_TOGGLE_DEFAULT);
    if key == toggle_key && toggle_key != "key.keyboard.unknown" {
        m::toggle_enabled();
    }
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[
            m::KEY_TOGGLE.into(),
            m::KEY_TOGGLE_LABEL.into(),
            m::KEY_TOGGLE_DEFAULT.into(),
        ]],
    });
}

pub(super) fn on_settings(_game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != m::ID {
        return;
    }
    let enabled_str = if m::is_enabled() { "ON" } else { "OFF" };
    buttons.push(format!("Mod Status: {enabled_str}"));
    buttons.push(format!("Height: {}", m::mode_label()));
    buttons.push(format!("Opacity: {}", m::opacity_label()));
}

pub(super) fn on_click(_game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != m::ID {
        return;
    }
    match button {
        0 => {
            m::toggle_enabled();
        }
        1 => {
            m::cycle_mode();
        }
        2 => {
            m::cycle_opacity();
        }
        _ => {}
    }
}
