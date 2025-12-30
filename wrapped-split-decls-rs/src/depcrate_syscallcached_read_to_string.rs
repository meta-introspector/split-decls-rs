// Generated macro for cached_read_to_string (function)
macro_rules! Depcrate_syscallcached_read_to_string {
() => {
// Module: crate::syscall
// Provides: {"cached_read_to_string"}
// Dependencies: {}
pub fn cached_read_to_string < P : AsRef < Path > > (path : P) -> anyhow :: Result < String > { let mut cache = GLOBAL_CACHE . lock () . unwrap () ; let content = cache . get_cached_content (path . as_ref ()) ? ; let _ = cache . save () ; Ok (content) }
};
}
