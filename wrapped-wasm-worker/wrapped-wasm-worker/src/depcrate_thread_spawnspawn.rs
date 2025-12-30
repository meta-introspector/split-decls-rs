// Generated macro for spawn (function)
macro_rules! Depcrate_thread_spawnspawn {
() => {
// Module: crate::thread::spawn
// Provides: {"spawn"}
// Dependencies: {}
# [doc = " See [`std::thread::spawn()`]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the main thread does not support spawning threads, see"] # [doc = " [`web::has_spawn_support()`](crate::web::has_spawn_support)."] # [allow (clippy :: min_ident_chars , clippy :: type_repetition_in_bounds)] pub fn spawn < F , T > (f : F) -> JoinHandle < T > where F : FnOnce () -> T , F : Send + 'static , T : Send + 'static , { Builder :: new () . spawn (f) . expect ("failed to spawn thread") }
};
}
