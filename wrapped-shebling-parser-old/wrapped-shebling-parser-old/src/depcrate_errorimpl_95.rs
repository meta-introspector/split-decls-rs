// Generated macro for impl_95 (impl)
macro_rules! Depcrate_errorimpl_95 {
() => {
// Module: crate::error
// Provides: {"impl_95"}
// Dependencies: {}
impl nom :: error :: ParseError < Span < '_ > > for ParseError { fn from_error_kind (input : Span < '_ > , _kind : nom :: error :: ErrorKind) -> Self { Self { location : Location :: from (& input) , notes : vec ! [] , diags : input . extra . take_diags () , } } fn append (_input : Span < '_ > , _kind : nom :: error :: ErrorKind , other : Self) -> Self { other } }
};
}
