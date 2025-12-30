// Generated macro for parse_from_modifier_value (function)
macro_rules! Depcrate_format_description_parse_format_itemparse_from_modifier_value {
() => {
// Module: crate::format_description::parse::format_item
// Provides: {"parse_from_modifier_value"}
// Dependencies: {}
# [doc = " Parse a modifier value using `FromStr`. Requires the modifier value to be valid UTF-8."] # [inline] fn parse_from_modifier_value < T : FromStr > (value : & Spanned < & [u8] >) -> Result < Option < T > , Error > { str :: from_utf8 (value) . ok () . and_then (| val | val . parse :: < T > () . ok ()) . map (| val | Some (val)) . ok_or_else (| | Error { _inner : unused (value . span . error ("invalid modifier value")) , public : crate :: error :: InvalidFormatDescription :: InvalidModifier { value : String :: from_utf8_lossy (value) . into_owned () , index : value . span . start . byte as usize , } , }) }
};
}
