// Generated macro for x448 (function)
macro_rules! Depcratex448 {
() => {
// Module: crate
// Provides: {"x448"}
// Dependencies: {}
# [doc = " A safe version of the x448 function defined in RFC448."] # [doc = " Currently, the only reason I can think of for using the raw function is FFI."] # [doc = " Option is FFI safe[1]. So we can still maintain that the invariant that"] # [doc = " we do not return a low order point."] # [doc = ""] # [doc = " [1]: https://github.com/rust-lang/nomicon/issues/59"] pub fn x448 (scalar_bytes : [u8 ; 56] , point_bytes : [u8 ; 56]) -> Option < [u8 ; 56] > { let point = PublicKey :: from_bytes (& point_bytes) ? ; let scalar = EphemeralSecret :: new (scalar_bytes . into ()) . as_scalar () ; Some ((& point . 0 * & scalar) . 0) }
};
}
