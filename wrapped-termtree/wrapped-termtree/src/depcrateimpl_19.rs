// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < D : Display > Extend < Tree < D > > for Tree < D > { fn extend < T : IntoIterator < Item = Tree < D > > > (& mut self , iter : T) { self . leaves . extend (iter) ; } }
};
}
