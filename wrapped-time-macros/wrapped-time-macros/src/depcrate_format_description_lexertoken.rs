// Generated macro for Token (enum)
macro_rules! Depcrate_format_description_lexerToken {
() => {
// Module: crate::format_description::lexer
// Provides: {"Token"}
// Dependencies: {}
pub (super) enum Token < 'a > { Literal (Spanned < & 'a [u8] >) , Bracket { kind : BracketKind , location : Location , } , ComponentPart { kind : ComponentKind , value : Spanned < & 'a [u8] > , } , }
};
}
