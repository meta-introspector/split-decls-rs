// Generated macro for cast_target (function)
macro_rules! Depcrate_callconv_x86_64cast_target {
() => {
// Module: crate::callconv::x86_64
// Provides: {"cast_target"}
// Dependencies: {}
fn cast_target (cls : & [Option < Class >] , size : Size) -> CastTarget { let mut i = 0 ; let lo = reg_component (cls , & mut i , size) . unwrap () ; let offset = Size :: from_bytes (8) * (i as u64) ; let mut target = CastTarget :: from (lo) ; if size > offset { if let Some (hi) = reg_component (cls , & mut i , size - offset) { target = CastTarget :: pair (lo , hi) ; } } assert_eq ! (reg_component (cls , & mut i , Size :: ZERO) , None) ; target }
};
}
