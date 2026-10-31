fn low_fire_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::mod_low_fire_glue::on_key(game, key, pressed);
}
