// Generated macro for macro_45 (macro)
macro_rules! Depcrate_detect_cachemacro_45 {
() => {
// Module: crate::detect::cache
// Provides: {"macro_45"}
// Dependencies: {}
cfg_select ! { feature = "std_detect_env_override" => { # [inline] fn disable_features (disable : & [u8] , value : & mut Initializer) { if let Ok (disable) = core :: str :: from_utf8 (disable) { for v in disable . split (" ") { let _ = super :: Feature :: from_str (v) . map (| v | value . unset (v as u32)) ; } } } # [inline] fn initialize (mut value : Initializer) -> Initializer { use core :: ffi :: CStr ; const RUST_STD_DETECT_UNSTABLE : & CStr = c"RUST_STD_DETECT_UNSTABLE" ; cfg_select ! { windows => { use alloc :: vec ; # [link (name = "kernel32")] unsafe extern "system" { fn GetEnvironmentVariableA (name : * const u8 , buffer : * mut u8 , size : u32) -> u32 ; } let len = unsafe { GetEnvironmentVariableA (RUST_STD_DETECT_UNSTABLE . as_ptr () . cast ::< u8 > () , core :: ptr :: null_mut () , 0) } ; if len > 0 { let mut env = vec ! [0 ; len as usize + 1] ; let len = unsafe { GetEnvironmentVariableA (RUST_STD_DETECT_UNSTABLE . as_ptr () . cast ::< u8 > () , env . as_mut_ptr () , len + 1) } ; if len > 0 { disable_features (& env [.. len as usize] , & mut value) ; } } } _ => { let env = unsafe { libc :: getenv (RUST_STD_DETECT_UNSTABLE . as_ptr ()) } ; if ! env . is_null () { let len = unsafe { libc :: strlen (env) } ; let env = unsafe { core :: slice :: from_raw_parts (env as * const u8 , len) } ; disable_features (env , & mut value) ; } } } do_initialize (value) ; value } } _ => { # [inline] fn initialize (value : Initializer) -> Initializer { do_initialize (value) ; value } } }
};
}
