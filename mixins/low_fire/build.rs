fn low_fire_build(
    this: &crate::screen_effects::ScreenEffects,
    input: &crate::screen_effects::ScreenInputs,
) -> core::ops::ControlFlow<Vec<ScreenQuad>> {
    let _ = this;
    if !crate::mod_low_fire::is_enabled()
        || !input.on_fire
        || crate::mod_low_fire::mode() == crate::mod_low_fire::LowFireMode::Vanilla
    {
        return core::ops::ControlFlow::Continue(());
    }

    use crate::hud_layout::GuiDraw;
    use crate::screen_effects::{Blend, ScreenQuad, ScreenVertex};

    let mut out = Vec::new();
    let sprite = |name: &str| input.sprites.get(name).map(|r| [r.u0, r.v0, r.u1, r.v1]);
    let bit = |on: bool, b: u32| if on { b } else { 0 };
    let texture_sampling = |name: &str| {
        input
            .flags
            .get(name)
            .copied()
            .map_or(0, |f| bit(f.blur, 1) | bit(f.clamp, 2))
    };

    let half = 0.5 * (35.0f32).to_radians().tan();
    let scale = [1.0 / (half * input.aspect), 1.0 / half];

    let hand_quad = |corners: [[f32; 2]; 4],
                     uv: [[f32; 2]; 4],
                     rect: [f32; 4],
                     color: [f32; 4],
                     sampling: u32| ScreenQuad {
        vertices: std::array::from_fn(|i| ScreenVertex {
            pos: [corners[i][0] * scale[0], corners[i][1] * scale[1]],
            uv: uv[i],
            rect,
            color,
        }),
        blend: Blend::Alpha,
        sampling,
    };
    let full = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];

    // 1. Block overlay
    if let Some(name) = &input.block_overlay
        && let Some(rect) = sprite(name)
    {
        let c = 0x19 as f32 / 255.0;
        out.push(hand_quad(
            full,
            [[1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
            rect,
            [c, c, c, 1.0],
            0,
        ));
    }

    // 2. Underwater
    if let Some((brightness, yaw, pitch)) = input.water
        && let Some(rect) = sprite("misc/underwater")
    {
        let (u, v) = (-yaw / 64.0, pitch / 64.0);
        let uv = [[u + 4.0, v + 4.0], [u, v + 4.0], [u, v], [u + 4.0, v]];
        out.push(hand_quad(
            full,
            uv,
            rect,
            [brightness, brightness, brightness, 0.1],
            texture_sampling("misc/underwater"),
        ));
    }

    // 3. Fire overlay (LOW FIRE MODIFIED)
    let mode = crate::mod_low_fire::mode();
    if mode != crate::mod_low_fire::LowFireMode::NoFire {
        if let Some(rect) = sprite("block/fire_1") {
            let y_offset = crate::mod_low_fire::fire_y_offset();
            let alpha = (0xE5 as f32 / 255.0) * crate::mod_low_fire::fire_opacity_factor();

            for (dx, angle) in [(0.24f32, -10.0f32), (-0.24, 10.0)] {
                let (s, c) = angle.to_radians().sin_cos();
                let corner = |x: f32, y: f32| [x * c + -0.5 * s + dx, y - 0.3 - y_offset];
                let corners = [
                    corner(-0.5, -0.5),
                    corner(0.5, -0.5),
                    corner(0.5, 0.5),
                    corner(-0.5, 0.5),
                ];
                out.push(hand_quad(
                    corners,
                    [[1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
                    rect,
                    [1.0, 1.0, 1.0, alpha],
                    0,
                ));
            }
        }
    }

    // 4. GUI camera overlays
    let gui = |rect: [f32; 4], color: [f32; 4], blend: Blend, sampling: u32| ScreenQuad {
        vertices: std::array::from_fn(|i| {
            let p = [[-1.0f32, 1.0], [1.0, 1.0], [1.0, -1.0], [-1.0, -1.0]][i];
            ScreenVertex {
                pos: p,
                uv: [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]][i],
                rect,
                color,
            }
        }),
        blend,
        sampling,
    };
    let rgba = |c: u32| {
        let [a, r, g, b] = c.to_be_bytes();
        [r, g, b, a].map(|v| f32::from(v) / 255.0)
    };

    for d in input.gui_overlays {
        match d {
            GuiDraw::Blit {
                texture,
                blend,
                x: 0,
                y: 0,
                color,
                ..
            } if *blend != crate::hud_layout::Blend::Nausea
                && texture != "misc/spyglass_scope" =>
            {
                let blend = if *blend == crate::hud_layout::Blend::Vignette {
                    Blend::Vignette
                } else {
                    Blend::Alpha
                };
                if blend == Blend::Vignette && color & 0x00ff_ffff == 0 {
                    continue;
                }
                if let Some(rect) = sprite(texture) {
                    out.push(gui(rect, rgba(*color), blend, texture_sampling(texture)));
                }
            }
            GuiDraw::AtlasSprite {
                sprite: name,
                color,
                ..
            } => {
                if let Some(rect) = sprite(name) {
                    out.push(gui(rect, rgba(*color), Blend::Alpha, 0));
                }
            }
            _ => {}
        }
    }

    core::ops::ControlFlow::Break(out)
}
