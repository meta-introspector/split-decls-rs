// Generated macro for other_414 (other)
macro_rules! Depcrate_randomother_414 {
() => {
// Module: crate::random
// Provides: {"other_414"}
// Dependencies: {}
extern "C" { pub static kSecRandomDefault : SecRandomRef ; pub fn SecRandomCopyBytes (rnd : SecRandomRef , count : usize , bytes : * mut c_void) -> c_int ; }
};
}
