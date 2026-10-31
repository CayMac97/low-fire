fn low_fire_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::mod_low_fire_glue::on_frame(game, elapsed);
}
