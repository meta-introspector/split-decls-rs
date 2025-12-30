// Generated macro for impl_102 (impl)
macro_rules! Depcrate_contextimpl_102 {
() => {
// Module: crate::context
// Provides: {"impl_102"}
// Dependencies: {}
# [doc = " Implement error handling for extracting function ABI information."] impl < 'tcx , B : Bridge > FnAbiOfHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { type FnAbiOfResult = Result < & 'tcx rustc_target :: callconv :: FnAbi < 'tcx , Ty < 'tcx > > , B :: Error > ; # [inline] fn handle_fn_abi_err (& self , err : ty :: layout :: FnAbiError < 'tcx > , _span : rustc_span :: Span , fn_abi_request : ty :: layout :: FnAbiRequest < 'tcx > ,) -> B :: Error { B :: Error :: new (format ! ("Failed to get ABI for `{fn_abi_request:?}`: {err:?}")) } }
};
}
