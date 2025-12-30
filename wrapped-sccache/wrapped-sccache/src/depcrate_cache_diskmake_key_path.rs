// Generated macro for make_key_path (function)
macro_rules! Depcrate_cache_diskmake_key_path {
() => {
// Module: crate::cache::disk
// Provides: {"make_key_path"}
// Dependencies: {}
# [doc = " Make a path to the cache entry with key `key`."] fn make_key_path (key : & str) -> PathBuf { Path :: new (& key [0 .. 1]) . join (& key [1 .. 2]) . join (key) }
};
}
