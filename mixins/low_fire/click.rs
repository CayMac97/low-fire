fn low_fire_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    crate::game::mod_low_fire_glue::on_click(game, mod_id, button);
}
