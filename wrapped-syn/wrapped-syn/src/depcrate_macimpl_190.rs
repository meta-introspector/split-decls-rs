// Generated macro for impl_190 (impl)
macro_rules! Depcrate_macimpl_190 {
() => {
// Module: crate::mac
// Provides: {"impl_190"}
// Dependencies: {}
impl Mac { pub fn is_braced (& self) -> bool { match self . tokens . last () { Some (t) => t . is_braced () , None => false , } } }
};
}
