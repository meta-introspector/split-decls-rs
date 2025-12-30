// Generated macro for impl_copy_clone (macro)
macro_rules! Depcrate_de_valueimpl_copy_clone {
() => {
// Module: crate::de::value
// Provides: {"impl_copy_clone"}
// Dependencies: {}
macro_rules ! impl_copy_clone { ($ ty : ident $ (<$ lifetime : tt >) *) => { impl <$ ($ lifetime ,) * E > Copy for $ ty <$ ($ lifetime ,) * E > { } impl <$ ($ lifetime ,) * E > Clone for $ ty <$ ($ lifetime ,) * E > { fn clone (& self) -> Self { * self } } } ; }
};
}
