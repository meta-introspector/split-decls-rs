// Generated macro for impl_509 (impl)
macro_rules! Depcrate_stream_tokenimpl_509 {
() => {
// Module: crate::stream::token
// Provides: {"impl_509"}
// Dependencies: {}
impl < T , O > Compare < O > for TokenSlice < '_ , T > where T : PartialEq < O > + Eq , { # [inline] fn compare (& self , t : O) -> CompareResult { if let Some (token) = self . first () { if * token == t { CompareResult :: Ok (1) } else { CompareResult :: Error } } else { CompareResult :: Incomplete } } }
};
}
