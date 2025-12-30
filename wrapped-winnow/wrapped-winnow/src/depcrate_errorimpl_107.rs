// Generated macro for impl_107 (impl)
macro_rules! Depcrate_errorimpl_107 {
() => {
// Module: crate::error
// Provides: {"impl_107"}
// Dependencies: {}
impl core :: fmt :: Display for ContextError < StrContext > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { # [cfg (feature = "alloc")] { let expression = self . context () . find_map (| c | match c { StrContext :: Label (c) => Some (c) , _ => None , }) ; let expected = self . context () . filter_map (| c | match c { StrContext :: Expected (c) => Some (c) , _ => None , }) . collect :: < alloc :: vec :: Vec < _ > > () ; let mut newline = false ; if let Some (expression) = expression { newline = true ; write ! (f , "invalid {expression}") ? ; } if ! expected . is_empty () { if newline { writeln ! (f) ? ; } newline = true ; write ! (f , "expected ") ? ; for (i , expected) in expected . iter () . enumerate () { if i != 0 { write ! (f , ", ") ? ; } write ! (f , "{expected}") ? ; } } # [cfg (feature = "std")] { if let Some (cause) = self . cause () { if newline { writeln ! (f) ? ; } write ! (f , "{cause}") ? ; } } } Ok (()) } }
};
}
