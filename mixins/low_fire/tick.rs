fn low_fire_tick(game: &mut crate::game::Game<'_>) {
    crate::game::mod_low_fire_glue::on_tick(game);
}
