// Generated macro for perform_call (function)
macro_rules! Depcrate_unix_apple_macos_component_x86perform_call {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"perform_call"}
// Dependencies: {}
unsafe fn perform_call (conn : io_connect_t , index : c_int , input_structure : * const ffi :: KeyData_t , output_structure : * mut ffi :: KeyData_t ,) -> i32 { let mut structure_output_size = mem :: size_of :: < ffi :: KeyData_t > () ; unsafe { IOConnectCallStructMethod (conn , index as u32 , input_structure . cast () , mem :: size_of :: < ffi :: KeyData_t > () , output_structure . cast () , & mut structure_output_size ,) } }
};
}
