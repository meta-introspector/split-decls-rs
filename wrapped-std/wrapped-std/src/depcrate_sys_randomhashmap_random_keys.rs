// Generated macro for hashmap_random_keys (function)
macro_rules! Depcrate_sys_randomhashmap_random_keys {
() => {
// Module: crate::sys::random
// Provides: {"hashmap_random_keys"}
// Dependencies: {}
# [cfg (not (any (target_os = "linux" , target_os = "android" , all (target_family = "wasm" , target_os = "unknown") , all (target_os = "wasi" , target_env = "p2") , target_os = "xous" ,)))] pub fn hashmap_random_keys () -> (u64 , u64) { let mut buf = [0 ; 16] ; fill_bytes (& mut buf) ; let k1 = u64 :: from_ne_bytes (buf [.. 8] . try_into () . unwrap ()) ; let k2 = u64 :: from_ne_bytes (buf [8 ..] . try_into () . unwrap ()) ; (k1 , k2) }
};
}
