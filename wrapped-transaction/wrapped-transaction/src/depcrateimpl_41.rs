// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl Sanitize for Transaction { fn sanitize (& self) -> result :: Result < () , SanitizeError > { if self . message . header . num_required_signatures as usize > self . signatures . len () { return Err (SanitizeError :: IndexOutOfBounds) ; } if self . signatures . len () > self . message . account_keys . len () { return Err (SanitizeError :: IndexOutOfBounds) ; } self . message . sanitize () } }
};
}
