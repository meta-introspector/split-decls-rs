// Generated macro for impl_439 (impl)
macro_rules! Depcrate_genericsimpl_439 {
() => {
// Module: crate::generics
// Provides: {"impl_439"}
// Dependencies: {}
# [cfg (feature = "printing")] impl < 'a > TypeGenerics < 'a > { # [doc = " Turn a type's generics like `<X, Y>` into a turbofish like `::<X, Y>`."] pub fn as_turbofish (& self) -> Turbofish < 'a > { Turbofish (self . 0) } }
};
}
