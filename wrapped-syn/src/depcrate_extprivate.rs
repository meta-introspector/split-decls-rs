// Generated macro for private (module)
macro_rules! Depcrate_extprivate {
() => {
// Module: crate::ext
// Provides: {"private"}
// Dependencies: {}
mod private { use proc_macro2 :: Ident ; pub trait Sealed { } impl Sealed for Ident { } # [cfg (feature = "parsing")] pub struct PeekFn ; # [cfg (feature = "parsing")] pub struct IdentAny ; # [cfg (feature = "parsing")] impl Copy for PeekFn { } # [cfg (feature = "parsing")] impl Clone for PeekFn { fn clone (& self) -> Self { * self } } }
};
}
