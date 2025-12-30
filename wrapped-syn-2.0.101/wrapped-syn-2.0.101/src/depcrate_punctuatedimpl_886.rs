// Generated macro for impl_886 (impl)
macro_rules! Depcrate_punctuatedimpl_886 {
() => {
// Module: crate::punctuated
// Provides: {"impl_886"}
// Dependencies: {}
impl < T , P > Clone for IntoPairs < T , P > where T : Clone , P : Clone , { fn clone (& self) -> Self { IntoPairs { inner : self . inner . clone () , last : self . last . clone () , } } }
};
}
