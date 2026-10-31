fn low_fire_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_low_fire_glue::on_list(mods);
}
