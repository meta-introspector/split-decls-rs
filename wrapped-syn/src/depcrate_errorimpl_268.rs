// Generated macro for impl_268 (impl)
macro_rules! Depcrate_errorimpl_268 {
() => {
// Module: crate::error
// Provides: {"impl_268"}
// Dependencies: {}
impl From < LexError > for Error { fn from (err : LexError) -> Self { Error :: new (err . span () , err) } }
};
}
