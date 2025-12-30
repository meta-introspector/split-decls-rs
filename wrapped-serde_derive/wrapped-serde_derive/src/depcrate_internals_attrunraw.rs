// Generated macro for unraw (function)
macro_rules! Depcrate_internals_attrunraw {
() => {
// Module: crate::internals::attr
// Provides: {"unraw"}
// Dependencies: {}
fn unraw (ident : & Ident) -> Ident { Ident :: new (ident . to_string () . trim_start_matches ("r#") , ident . span ()) }
};
}
