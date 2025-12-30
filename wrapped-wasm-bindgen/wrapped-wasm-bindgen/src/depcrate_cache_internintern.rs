// Generated macro for intern (function)
macro_rules! Depcrate_cache_internintern {
() => {
// Module: crate::cache::intern
// Provides: {"intern"}
// Dependencies: {}
# [doc = " Interns Rust strings so that it's much faster to send them to JS."] # [doc = ""] # [doc = " Sending strings from Rust to JS is slow, because it has to do a full `O(n)`"] # [doc = " copy and *also* encode from UTF-8 to UTF-16. This must be done every single"] # [doc = " time a string is sent to JS."] # [doc = ""] # [doc = " If you are sending the same string multiple times, you can call this `intern`"] # [doc = " function, which simply returns its argument unchanged:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use wasm_bindgen::intern;"] # [doc = " intern(\"foo\") // returns \"foo\""] # [doc = " # ;"] # [doc = " ```"] # [doc = ""] # [doc = " However, if you enable the `\"enable-interning\"` feature for wasm-bindgen,"] # [doc = " then it will add the string into an internal cache."] # [doc = ""] # [doc = " When you send that cached string to JS, it will look it up in the cache,"] # [doc = " which completely avoids the `O(n)` copy and encoding. This has a significant"] # [doc = " speed boost (as high as 783%)!"] # [doc = ""] # [doc = " However, there is a small cost to this caching, so you shouldn't cache every"] # [doc = " string. Only cache strings which have a high likelihood of being sent"] # [doc = " to JS multiple times."] # [doc = ""] # [doc = " Also, keep in mind that this function is a *performance hint*: it's not"] # [doc = " *guaranteed* that the string will be cached, and the caching strategy"] # [doc = " might change at any time, so don't rely upon it."] # [inline] pub fn intern (s : & str) -> & str { # [cfg (feature = "enable-interning")] intern_str (s) ; s }
};
}
