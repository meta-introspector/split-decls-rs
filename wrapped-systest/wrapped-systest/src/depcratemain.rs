// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { println ! ("curl version is {:?}!" , curl :: Version :: get () . version ()) ; unsafe { println ! ("zlib version is {:?}!" , CStr :: from_ptr (libz_sys :: zlibVersion ())) ; } openssl_sys :: init () ; test_extras () ; }
};
}
