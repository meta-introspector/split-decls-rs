// Generated macro for impl_903 (impl)
macro_rules! Depcrate_punctuatedimpl_903 {
() => {
// Module: crate::punctuated
// Provides: {"impl_903"}
// Dependencies: {}
impl < T , P > Clone for IntoPairs < T , P > where T : Clone , P : Clone , { fn clone (& self) -> Self { IntoPairs { inner : self . inner . clone () , last : self . last . clone () , } } }
};
}
