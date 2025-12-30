// Generated macro for impl_273 (impl)
macro_rules! Depcrate_errorimpl_273 {
() => {
// Module: crate::error
// Provides: {"impl_273"}
// Dependencies: {}
impl Extend < Error > for Error { fn extend < T : IntoIterator < Item = Error > > (& mut self , iter : T) { for err in iter { self . combine (err) ; } } }
};
}
