// Generated macro for impl_9 (impl)
macro_rules! Depcrate_oneshotimpl_9 {
() => {
// Module: crate::oneshot
// Provides: {"impl_9"}
// Dependencies: {}
impl WorkerFnType for OneshotFn { type OutputType = Type ; type RecvType = Type ; fn attr_name () -> & 'static str { "oneshot" } fn worker_type_name () -> & 'static str { "oneshot" } fn parse_recv_type (sig : & Signature) -> syn :: Result < Self :: RecvType > { let mut inputs = sig . inputs . iter () ; let arg = inputs . next () . ok_or_else (| | syn :: Error :: new_spanned (& sig . ident , "expected 1 argument")) ? ; let ty = Self :: extract_fn_arg_type (arg) ? ; Self :: assert_no_left_argument (inputs , 1) ? ; Ok (ty) } fn parse_output_type (sig : & Signature) -> syn :: Result < Self :: OutputType > { let ty = match & sig . output { ReturnType :: Default => { parse_quote ! { () } } ReturnType :: Type (_ , ty) => * ty . clone () , } ; Ok (ty) } }
};
}
