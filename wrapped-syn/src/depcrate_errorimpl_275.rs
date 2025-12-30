// Generated macro for impl_275 (impl)
macro_rules! Depcrate_errorimpl_275 {
() => {
// Module: crate::error
// Provides: {"impl_275"}
// Dependencies: {}
impl Extend < Error > for Error { fn extend < T : IntoIterator < Item = Error > > (& mut self , iter : T) { for err in iter { self . combine (err) ; } } }
};
}
