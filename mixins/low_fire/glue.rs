// Glue between Horizon's `Game` and `mod_low_fire`
use crate::game::Game;
use crate::game::mod_low_fire as m;
use crate::game::native_hooks::NativeMod;
use horizon_client::extras_hud::Corners;
use horizon_client::hud::HudBatch;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::Duration;

static SAVED_FIRE_TICKS: AtomicI32 = AtomicI32::new(0);

fn restore_fire_ticks(game: &mut Game<'_>) {
    let saved = SAVED_FIRE_TICKS.load(Ordering::Relaxed);
    if saved > 0 {
        if let Some(p) = game.player.as_mut() {
            if p.fire_ticks <= 0 {
                p.fire_ticks = saved;
            }
        }
    }
}

pub(super) fn on_frame(game: &mut Game<'_>, _elapsed: Duration) {
    if !game.native_on(m::ID) || !m::is_enabled() {
        return;
    }
    restore_fire_ticks(game);
}

pub(super) fn on_tick(game: &mut Game<'_>) {
    if !game.native_on(m::ID) || !m::is_enabled() {
        return;
    }
    restore_fire_ticks(game);
}

fn draw_low_fire(game: &Game<'_>, batch: &mut HudBatch) {
    let atlas = &game.gui.atlas;
    let sprite_a = atlas.sprites.get("block/fire_0").copied();
    let sprite_b = atlas.sprites.get("block/fire_1").copied();
    let sprite = sprite_b.or(sprite_a);
    let Some(r) = sprite else {
        return;
    };

    let sw = batch.width;
    let sh = batch.height;
    let height_ratio = match m::mode() {
        m::LowFireMode::Low => 0.22,
        m::LowFireMode::VeryLow => 0.14,
        m::LowFireMode::Minimal => 0.08,
        _ => 0.22,
    };
    let fire_h = (sh * height_ratio).max(16.0);
    let tile_w = fire_h * 1.25;
    let y = sh - fire_h;
    let alpha_byte = match m::opacity() {
        m::LowFireOpacity::Full => 230,
        m::LowFireOpacity::Subtle => 170,
        m::LowFireOpacity::Translucent => 110,
    };
    let color = horizon_client::hud::rgba(255, 255, 255, alpha_byte);

    let count = (sw / tile_w).ceil() as usize + 1;
    let time_sec = game.world.game_time as f32 * 0.1;

    for i in 0..count {
        let x = (i as f32) * tile_w;
        let use_b = ((i as f32 + time_sec) as usize) % 2 == 0;
        let cur_r = if use_b && sprite_b.is_some() {
            sprite_b.unwrap()
        } else {
            r
        };

        if i % 2 == 1 {
            batch.quad(
                [[x, y], [x + tile_w, y], [x + tile_w, y + fire_h], [x, y + fire_h]],
                [[cur_r.u1, cur_r.v0], [cur_r.u0, cur_r.v0], [cur_r.u0, cur_r.v1], [cur_r.u1, cur_r.v1]],
                color,
            );
        } else {
            batch.rect_uv(x, y, tile_w, fire_h, cur_r, color);
        }
    }
}

pub(super) fn on_hud(game: &mut Game<'_>, batch: &mut HudBatch, _corners: &mut Corners) {
    if !game.native_on(m::ID) || !m::is_enabled() {
        return;
    }

    if !game.camera_type.first_person() || game.spectator() {
        return;
    }

    let Some(p) = game.player.as_mut() else {
        return;
    };

    let burning_ticks = p.fire_ticks.max(SAVED_FIRE_TICKS.load(Ordering::Relaxed));
    if burning_ticks <= 0 {
        SAVED_FIRE_TICKS.store(0, Ordering::Relaxed);
        return;
    }

    SAVED_FIRE_TICKS.store(burning_ticks, Ordering::Relaxed);

    if m::mode() != m::LowFireMode::Vanilla {
        // Suppress full-screen fire overlay in screen_quads
        p.fire_ticks = 0;

        if m::mode() != m::LowFireMode::NoFire {
            draw_low_fire(game, batch);
        }
    }
}

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
