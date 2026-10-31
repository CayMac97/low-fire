fn low_fire_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_low_fire_glue::on_settings(game, mod_id, buttons);
}
