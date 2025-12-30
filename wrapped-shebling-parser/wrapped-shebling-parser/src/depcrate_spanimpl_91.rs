// Generated macro for impl_91 (impl)
macro_rules! Depcrate_spanimpl_91 {
() => {
// Module: crate::span
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'a > nom :: InputTake for ParseSpan < 'a > { # [inline] fn take (& self , count : usize) -> Self { self . slice (.. count) } # [inline] fn take_split (& self , count : usize) -> (Self , Self) { (self . slice (count ..) , self . slice (.. count)) } }
};
}
