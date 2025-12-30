// Generated macro for impl_74 (impl)
macro_rules! Depcrate_genericsimpl_74 {
() => {
// Module: crate::generics
// Provides: {"impl_74"}
// Dependencies: {}
# [cfg (feature = "printing")] impl < 'a > TyGenerics < 'a > { # [doc = " Turn a type's generics like `<X, Y>` into a turbofish like `::<X, Y>`."] pub fn as_turbofish (& self) -> Turbofish { Turbofish (self . 0) } }
};
}
