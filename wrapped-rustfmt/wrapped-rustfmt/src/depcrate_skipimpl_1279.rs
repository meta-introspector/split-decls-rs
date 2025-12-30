// Generated macro for impl_1279 (impl)
macro_rules! Depcrate_skipimpl_1279 {
() => {
// Module: crate::skip
// Provides: {"impl_1279"}
// Dependencies: {}
impl Extend < String > for SkipNameContext { fn extend < T : IntoIterator < Item = String > > (& mut self , iter : T) { match self { Self :: All => { } Self :: Values (values) => values . extend (iter) , } } }
};
}
