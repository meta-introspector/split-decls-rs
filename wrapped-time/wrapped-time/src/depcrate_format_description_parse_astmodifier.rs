// Generated macro for Modifier (struct)
macro_rules! Depcrate_format_description_parse_astModifier {
() => {
// Module: crate::format_description::parse::ast
// Provides: {"Modifier"}
// Dependencies: {}
# [doc = " A modifier for a component."] pub (super) struct Modifier < 'a > { # [doc = " Whitespace preceding the modifier."] pub (super) _leading_whitespace : Unused < Spanned < & 'a [u8] > > , # [doc = " The key of the modifier."] pub (super) key : Spanned < & 'a [u8] > , # [doc = " Where the colon of the modifier was in the format string."] pub (super) _colon : Unused < Location > , # [doc = " The value of the modifier."] pub (super) value : Spanned < & 'a [u8] > , }
};
}
