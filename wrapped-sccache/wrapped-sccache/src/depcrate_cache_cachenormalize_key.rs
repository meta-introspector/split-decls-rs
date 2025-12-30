// Generated macro for normalize_key (function)
macro_rules! Depcrate_cache_cachenormalize_key {
() => {
// Module: crate::cache::cache
// Provides: {"normalize_key"}
// Dependencies: {}
# [doc = " Normalize key `abcdef` into `a/b/c/abcdef`"] pub (in crate :: cache) fn normalize_key (key : & str) -> String { format ! ("{}/{}/{}/{}" , & key [0 .. 1] , & key [1 .. 2] , & key [2 .. 3] , & key) }
};
}
