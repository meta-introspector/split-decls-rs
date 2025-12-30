// Generated macro for macro_3507 (macro)
macro_rules! Depcrate_sys_personalitymacro_3507 {
() => {
// Module: crate::sys::personality
// Provides: {"macro_3507"}
// Dependencies: {}
# [cfg (not (any (test , doctest)))] cfg_select ! { target_os = "emscripten" => { mod emcc ; } any (target_env = "msvc" , target_family = "wasm") => { # [lang = "eh_personality"] fn rust_eh_personality () { core :: intrinsics :: abort () } } any (all (target_family = "windows" , target_env = "gnu") , target_os = "psp" , target_os = "xous" , target_os = "solid_asp3" , all (target_family = "unix" , not (target_os = "espidf") , not (target_os = "l4re") , not (target_os = "nuttx")) , all (target_vendor = "fortanix" , target_env = "sgx") ,) => { mod gcc ; } _ => { } }
};
}
