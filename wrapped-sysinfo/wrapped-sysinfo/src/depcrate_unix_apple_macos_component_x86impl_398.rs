// Generated macro for impl_398 (impl)
macro_rules! Depcrate_unix_apple_macos_component_x86impl_398 {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"impl_398"}
// Dependencies: {}
impl ComponentFFI { fn new (key : & [i8] , connection : io_connect_t) -> Option < ComponentFFI > { unsafe { get_key_size (connection , key) . ok () . map (| (input_structure , val) | ComponentFFI { input_structure , val , connection , }) } } fn temperature (& self) -> Option < f32 > { get_temperature_inner (self . connection , & self . input_structure , & self . val) } }
};
}
