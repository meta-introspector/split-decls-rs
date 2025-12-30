// Generated macro for Token (enum)
macro_rules! Depcrate_format_description_parse_strftimeToken {
() => {
// Module: crate::format_description::parse::strftime
// Provides: {"Token"}
// Dependencies: {}
enum Token < 'a > { Literal (Spanned < & 'a [u8] >) , Component { _percent : Unused < Location > , padding : Spanned < Padding > , component : Spanned < u8 > , } , }
};
}
