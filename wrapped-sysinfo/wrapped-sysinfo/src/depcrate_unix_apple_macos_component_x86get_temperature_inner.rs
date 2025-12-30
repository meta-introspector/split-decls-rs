// Generated macro for get_temperature_inner (function)
macro_rules! Depcrate_unix_apple_macos_component_x86get_temperature_inner {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"get_temperature_inner"}
// Dependencies: {}
fn get_temperature_inner (con : io_connect_t , input_structure : & ffi :: KeyData_t , original_val : & ffi :: Val_t ,) -> Option < f32 > { unsafe { if let Ok (val) = read_key (con , input_structure , (* original_val) . clone ()) { if val . data_size > 0 && libc :: strcmp (val . data_type . as_ptr () , c"sp78" . as_ptr () as * const i8) == 0 { let x = (i32 :: from (val . bytes [0]) << 6) + (i32 :: from (val . bytes [1]) >> 2) ; return Some (x as f32 / 64f32) ; } } } None }
};
}
