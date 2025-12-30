// Generated macro for input_type (function)
macro_rules! Depcrate_arbitraryinput_type {
() => {
// Module: crate::arbitrary
// Provides: {"input_type"}
// Dependencies: {}
fn input_type (expr : & Expr) -> Option < & Type > { if let Expr :: Closure (closure) = expr { let inputs = & closure . inputs ; if inputs . len () == 1 { if let Pat :: Type (t) = & inputs [0] { return Some (& t . ty) ; } } } None }
};
}
