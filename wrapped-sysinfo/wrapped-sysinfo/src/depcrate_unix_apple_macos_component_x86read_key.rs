// Generated macro for read_key (function)
macro_rules! Depcrate_unix_apple_macos_component_x86read_key {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"read_key"}
// Dependencies: {}
unsafe fn read_key (con : io_connect_t , input_structure : & ffi :: KeyData_t , mut val : ffi :: Val_t ,) -> Result < ffi :: Val_t , i32 > { unsafe { let mut output_structure : ffi :: KeyData_t = mem :: zeroed :: < ffi :: KeyData_t > () ; # [allow (non_upper_case_globals)] match perform_call (con , ffi :: KERNEL_INDEX_SMC , input_structure , & mut output_structure ,) { kIOReturnSuccess => { libc :: memcpy (val . bytes . as_mut_ptr () as * mut c_void , output_structure . bytes . as_mut_ptr () as * mut c_void , mem :: size_of :: < [u8 ; 32] > () ,) ; Ok (val) } result => Err (result) , } } }
};
}
