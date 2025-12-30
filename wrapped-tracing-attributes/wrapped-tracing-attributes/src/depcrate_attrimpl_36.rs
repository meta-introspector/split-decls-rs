// Generated macro for impl_36 (impl)
macro_rules! Depcrate_attrimpl_36 {
() => {
// Module: crate::attr
// Provides: {"impl_36"}
// Dependencies: {}
impl Parse for Field { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut kind = FieldKind :: Value ; if input . peek (Token ! [%]) { input . parse :: < Token ! [%] > () ? ; kind = FieldKind :: Display ; } else if input . peek (Token ! [?]) { input . parse :: < Token ! [?] > () ? ; kind = FieldKind :: Debug ; } ; let name = Punctuated :: parse_separated_nonempty_with (input , Ident :: parse_any) ? ; let value = if input . peek (Token ! [=]) { input . parse :: < Token ! [=] > () ? ; if input . peek (Token ! [%]) { input . parse :: < Token ! [%] > () ? ; kind = FieldKind :: Display ; } else if input . peek (Token ! [?]) { input . parse :: < Token ! [?] > () ? ; kind = FieldKind :: Debug ; } ; Some (input . parse () ?) } else { None } ; Ok (Self { name , value , kind }) } }
};
}
