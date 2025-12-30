// Generated macro for x448_unchecked (function)
macro_rules! Depcratex448_unchecked {
() => {
// Module: crate
// Provides: {"x448_unchecked"}
// Dependencies: {}
# [doc = " An unchecked version of the x448 function defined in RFC448"] # [doc = " No checks are made on the points."] pub fn x448_unchecked (scalar_bytes : [u8 ; 56] , point_bytes : [u8 ; 56]) -> [u8 ; 56] { let point = MontgomeryPoint (point_bytes) ; let scalar = EphemeralSecret :: new (scalar_bytes . into ()) . as_scalar () ; (& point * & scalar) . 0 }
};
}
