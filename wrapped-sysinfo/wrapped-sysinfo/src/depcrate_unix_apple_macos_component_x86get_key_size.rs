// Generated macro for get_key_size (function)
macro_rules! Depcrate_unix_apple_macos_component_x86get_key_size {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"get_key_size"}
// Dependencies: {}
unsafe fn get_key_size (con : io_connect_t , key : & [i8]) -> Result < (ffi :: KeyData_t , ffi :: Val_t) , i32 > { unsafe { let mut input_structure : ffi :: KeyData_t = mem :: zeroed :: < ffi :: KeyData_t > () ; let mut output_structure : ffi :: KeyData_t = mem :: zeroed :: < ffi :: KeyData_t > () ; let mut val : ffi :: Val_t = mem :: zeroed :: < ffi :: Val_t > () ; input_structure . key = strtoul (key) ; input_structure . data8 = ffi :: SMC_CMD_READ_KEYINFO ; let result = perform_call (con , ffi :: KERNEL_INDEX_SMC , & input_structure , & mut output_structure ,) ; if result != kIOReturnSuccess { return Err (result) ; } val . data_size = output_structure . key_info . data_size ; ultostr (val . data_type . as_mut_ptr () , output_structure . key_info . data_type ,) ; input_structure . key_info . data_size = val . data_size ; input_structure . data8 = ffi :: SMC_CMD_READ_BYTES ; Ok ((input_structure , val)) } }
};
}
