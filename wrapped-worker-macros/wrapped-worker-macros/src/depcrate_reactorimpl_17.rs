// Generated macro for impl_17 (impl)
macro_rules! Depcrate_reactorimpl_17 {
() => {
// Module: crate::reactor
// Provides: {"impl_17"}
// Dependencies: {}
impl WorkerFnType for ReactorFn { type OutputType = () ; type RecvType = Type ; fn attr_name () -> & 'static str { "reactor" } fn worker_type_name () -> & 'static str { "reactor" } fn parse_recv_type (sig : & Signature) -> syn :: Result < Self :: RecvType > { let mut inputs = sig . inputs . iter () ; let arg = inputs . next () . ok_or_else (| | syn :: Error :: new_spanned (& sig . ident , "expected 1 argument")) ? ; let ty = Self :: extract_fn_arg_type (arg) ? ; Self :: assert_no_left_argument (inputs , 1) ? ; Ok (ty) } fn parse_output_type (sig : & Signature) -> syn :: Result < Self :: OutputType > { match & sig . output { ReturnType :: Default => { } ReturnType :: Type (_ , ty) => { return Err (syn :: Error :: new_spanned (ty , "reactor workers cannot return any value" ,)) } } Ok (()) } }
};
}
