// Generated macro for impl_4 (impl)
macro_rules! Depcrate_errorimpl_4 {
() => {
// Module: crate::error
// Provides: {"impl_4"}
// Dependencies: {}
impl nom :: error :: ParseError < ParseSpan < '_ > > for ParseError { fn from_error_kind (input : ParseSpan , _kind : nom :: error :: ErrorKind) -> Self { Self { location : input . offset () , notes : vec ! [] , } } fn append (_input : ParseSpan , _kind : nom :: error :: ErrorKind , other : Self) -> Self { other } }
};
}
