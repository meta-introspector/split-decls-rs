// Generated macro for impl_266 (impl)
macro_rules! Depcrate_errorimpl_266 {
() => {
// Module: crate::error
// Provides: {"impl_266"}
// Dependencies: {}
impl From < LexError > for Error { fn from (err : LexError) -> Self { Error :: new (err . span () , err) } }
};
}
