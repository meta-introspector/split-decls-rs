// Generated macro for filter_array_elements_async (function)
macro_rules! Depcrate_needs_dropfilter_array_elements_async {
() => {
// Module: crate::needs_drop
// Provides: {"filter_array_elements_async"}
// Dependencies: {}
fn filter_array_elements_async < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> impl Fn (& Result < Ty < 'tcx > , AlwaysRequiresDrop >) -> bool { move | ty | match ty { Ok (ty) => match * ty . kind () { ty :: Array (elem , _) => tcx . needs_async_drop_raw (typing_env . as_query_input (elem)) , _ => true , } , Err (AlwaysRequiresDrop) => true , } }
};
}
