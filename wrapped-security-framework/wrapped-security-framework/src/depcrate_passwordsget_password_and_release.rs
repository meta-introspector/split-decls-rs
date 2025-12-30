// Generated macro for get_password_and_release (function)
macro_rules! Depcrate_passwordsget_password_and_release {
() => {
// Module: crate::passwords
// Provides: {"get_password_and_release"}
// Dependencies: {}
fn get_password_and_release (data : CFTypeRef) -> Result < Vec < u8 > > { if ! data . is_null () { let type_id = unsafe { CFGetTypeID (data) } ; if type_id == CFData :: type_id () { let val = unsafe { CFData :: wrap_under_create_rule (data as CFDataRef) } ; let mut vec = Vec :: new () ; if ! val . is_empty () { vec . extend_from_slice (val . bytes ()) ; } return Ok (vec) ; } unsafe { CFRelease (data) } ; } Err (Error :: from_code (errSecParam)) }
};
}
