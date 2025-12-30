// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < D : Display > Extend < D > for Tree < D > { fn extend < T : IntoIterator < Item = D > > (& mut self , iter : T) { self . leaves . extend (iter . into_iter () . map (Into :: into)) ; } }
};
}
