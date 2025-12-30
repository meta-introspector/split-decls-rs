// Generated macro for parse_ordinal (function)
macro_rules! Depcrate_parsing_componentparse_ordinal {
() => {
// Module: crate::parsing::component
// Provides: {"parse_ordinal"}
// Dependencies: {}
# [doc = " Parse the \"ordinal\" component of a `Date`."] # [inline] pub (crate) fn parse_ordinal (input : & [u8] , modifiers : modifier :: Ordinal ,) -> Option < ParsedItem < '_ , NonZero < u16 > > > { exactly_n_digits_padded :: < 3 , _ > (modifiers . padding) (input) }
};
}
