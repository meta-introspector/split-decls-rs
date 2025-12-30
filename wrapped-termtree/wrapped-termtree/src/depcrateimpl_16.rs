// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < D : Display > Tree < D > { pub fn push (& mut self , leaf : impl Into < Tree < D > >) -> & mut Self { self . leaves . push (leaf . into ()) ; self } }
};
}
