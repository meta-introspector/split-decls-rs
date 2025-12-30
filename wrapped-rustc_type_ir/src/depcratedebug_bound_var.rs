// Generated macro for debug_bound_var (function)
macro_rules! Depcratedebug_bound_var {
() => {
// Module: crate
// Provides: {"debug_bound_var"}
// Dependencies: {}
pub fn debug_bound_var < T : std :: fmt :: Write > (fmt : & mut T , debruijn : DebruijnIndex , var : impl std :: fmt :: Debug ,) -> Result < () , std :: fmt :: Error > { if debruijn == INNERMOST { write ! (fmt , "^{var:?}") } else { write ! (fmt , "^{}_{:?}" , debruijn . index () , var) } }
};
}
