// Generated macro for impl_784 (impl)
macro_rules! Depcrate_parse_quoteimpl_784 {
() => {
// Module: crate::parse_quote
// Provides: {"impl_784"}
// Dependencies: {}
# [cfg (any (feature = "full" , feature = "derive"))] impl ParseQuote for Field { fn parse (input : ParseStream) -> Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; let vis : Visibility = input . parse () ? ; let ident : Option < Ident > ; let colon_token : Option < Token ! [:] > ; let is_named = input . peek (Ident) && input . peek2 (Token ! [:]) && ! input . peek2 (Token ! [::]) ; if is_named { ident = Some (input . parse () ?) ; colon_token = Some (input . parse () ?) ; } else { ident = None ; colon_token = None ; } let ty : Type = input . parse () ? ; Ok (Field { attrs , vis , mutability : FieldMutability :: None , ident , colon_token , ty , }) } }
};
}
