// Generated macro for impl_373 (impl)
macro_rules! Depcrate_extimpl_373 {
() => {
// Module: crate::ext
// Provides: {"impl_373"}
// Dependencies: {}
impl IdentExt for Ident { fn parse_any (input : ParseStream) -> Result < Self > { input . step (| cursor | match cursor . ident () { Some ((ident , rest)) => Ok ((ident , rest)) , None => Err (cursor . error ("expected ident")) , }) } fn unraw (& self) -> Ident { let string = self . to_string () ; if let Some (string) = string . strip_prefix ("r#") { Ident :: new (string , self . span ()) } else { self . clone () } } }
};
}
