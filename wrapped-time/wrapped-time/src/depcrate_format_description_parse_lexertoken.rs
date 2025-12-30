// Generated macro for Token (enum)
macro_rules! Depcrate_format_description_parse_lexerToken {
() => {
// Module: crate::format_description::parse::lexer
// Provides: {"Token"}
// Dependencies: {}
# [doc = " A token emitted by the lexer. There is no semantic meaning at this stage."] pub (super) enum Token < 'a > { # [doc = " A literal string, formatted and parsed as-is."] Literal (Spanned < & 'a [u8] >) , # [doc = " An opening or closing bracket. May or may not be the start or end of a component."] Bracket { # [doc = " Whether the bracket is opening or closing."] kind : BracketKind , # [doc = " Where the bracket was in the format string."] location : Location , } , # [doc = " One part of a component. This could be its name, a modifier, or whitespace."] ComponentPart { # [doc = " Whether the part is whitespace or not."] kind : ComponentKind , # [doc = " The part itself."] value : Spanned < & 'a [u8] > , } , }
};
}
