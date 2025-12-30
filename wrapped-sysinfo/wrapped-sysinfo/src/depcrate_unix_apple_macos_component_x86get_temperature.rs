// Generated macro for get_temperature (function)
macro_rules! Depcrate_unix_apple_macos_component_x86get_temperature {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"get_temperature"}
// Dependencies: {}
fn get_temperature (con : io_connect_t , key : & [i8]) -> Option < f32 > { unsafe { let (input_structure , val) = get_key_size (con , key) . ok () ? ; get_temperature_inner (con , & input_structure , & val) } }
};
}
