// Generated macro for Registrable (trait)
macro_rules! Depcrate_traitsRegistrable {
() => {
// Module: crate::traits
// Provides: {"Registrable"}
// Dependencies: {}
# [doc = " A trait to enable public workers being registered in a web worker."] pub trait Registrable { # [doc = " Registrar Type."] type Registrar ; # [doc = " Creates a registrar for the current worker."] fn registrar () -> Self :: Registrar ; }
};
}
