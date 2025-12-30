// Generated macro for impl_1257 (impl)
macro_rules! Depcrate_skipimpl_1257 {
() => {
// Module: crate::skip
// Provides: {"impl_1257"}
// Dependencies: {}
impl Extend < String > for SkipNameContext { fn extend < T : IntoIterator < Item = String > > (& mut self , iter : T) { match self { Self :: All => { } Self :: Values (values) => values . extend (iter) , } } }
};
}
