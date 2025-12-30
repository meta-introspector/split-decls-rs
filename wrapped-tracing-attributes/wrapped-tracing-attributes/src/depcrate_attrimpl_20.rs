// Generated macro for impl_20 (impl)
macro_rules! Depcrate_attrimpl_20 {
() => {
// Module: crate::attr
// Provides: {"impl_20"}
// Dependencies: {}
impl Parse for EventArgs { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { if ! input . peek (syn :: token :: Paren) { return Ok (Self :: default ()) ; } let content ; let _ = syn :: parenthesized ! (content in input) ; let mut result = Self :: default () ; let mut parse_one_arg = | | { let lookahead = content . lookahead1 () ; if lookahead . peek (kw :: level) { if result . level . is_some () { return Err (content . error ("expected only a single `level` argument")) ; } result . level = Some (content . parse () ?) ; } else if result . mode != FormatMode :: default () { return Err (content . error ("expected only a single format argument")) ; } else if let Some (ident) = content . parse :: < Option < Ident > > () ? { match ident . to_string () . as_str () { "Debug" => result . mode = FormatMode :: Debug , "Display" => result . mode = FormatMode :: Display , _ => return Err (syn :: Error :: new (ident . span () , "unknown event formatting mode, expected either `Debug` or `Display`" ,)) , } } Ok (()) } ; parse_one_arg () ? ; if ! content . is_empty () { if content . lookahead1 () . peek (Token ! [,]) { let _ = content . parse :: < Token ! [,] > () ? ; parse_one_arg () ? ; } else { return Err (content . error ("expected `,` or `)`")) ; } } Ok (result) } }
};
}
