// Generated macro for WorkerFnType (trait)
macro_rules! Depcrate_worker_fnWorkerFnType {
() => {
// Module: crate::worker_fn
// Provides: {"WorkerFnType"}
// Dependencies: {}
pub trait WorkerFnType { type RecvType ; type OutputType ; fn attr_name () -> & 'static str ; fn worker_type_name () -> & 'static str ; fn parse_recv_type (sig : & Signature) -> syn :: Result < Self :: RecvType > ; fn parse_output_type (sig : & Signature) -> syn :: Result < Self :: OutputType > ; fn extract_fn_arg_type (arg : & FnArg) -> syn :: Result < Type > { let ty = match arg { FnArg :: Typed (arg) => arg . ty . clone () , FnArg :: Receiver (_) => { return Err (syn :: Error :: new_spanned (arg , format ! ("{} workers can't accept a receiver" , Self :: worker_type_name ()) ,)) ; } } ; Ok (* ty) } fn assert_no_left_argument < I , T > (rest_inputs : I , expected_len : usize) -> syn :: Result < () > where I : ExactSizeIterator + IntoIterator < Item = T > , T : ToTokens , { if rest_inputs . len () > 0 { let params : TokenStream = rest_inputs . into_iter () . map (| it | it . to_token_stream ()) . collect () ; return Err (syn :: Error :: new_spanned (params , format ! ("{} worker can accept at most {} argument{}" , Self :: worker_type_name () , expected_len , if expected_len > 1 { "s" } else { "" }) ,)) ; } Ok (()) } }
};
}
