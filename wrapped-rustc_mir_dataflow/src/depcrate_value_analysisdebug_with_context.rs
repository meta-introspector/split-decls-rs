// Generated macro for debug_with_context (function)
macro_rules! Depcrate_value_analysisdebug_with_context {
() => {
// Module: crate::value_analysis
// Provides: {"debug_with_context"}
// Dependencies: {}
pub fn debug_with_context < V : Debug + Eq + HasBottom > (new : & StateData < V > , old : Option < & StateData < V > > , map : & Map < '_ > , f : & mut Formatter < '_ > ,) -> std :: fmt :: Result { for (local , place) in map . locals . iter_enumerated () { if let Some (place) = place { debug_with_context_rec (* place , & format ! ("{local:?}") , new , old , map , f) ? ; } } Ok (()) }
};
}
