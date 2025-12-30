// Generated macro for parse_from_modifier_value (function)
macro_rules! Depcrate_format_description_format_itemparse_from_modifier_value {
() => {
// Module: crate::format_description::format_item
// Provides: {"parse_from_modifier_value"}
// Dependencies: {}
fn parse_from_modifier_value < T : FromStr > (value : & Spanned < & [u8] >) -> Result < Option < T > , Error > { str :: from_utf8 (value) . ok () . and_then (| val | val . parse :: < T > () . ok ()) . map (| val | Some (val)) . ok_or_else (| | value . span . error ("invalid modifier value")) }
};
}
