// Generated macro for parse_scoped (function)
macro_rules! Depcrate_parseparse_scoped {
() => {
// Module: crate::parse
// Provides: {"parse_scoped"}
// Dependencies: {}
pub (crate) fn parse_scoped < F : Parser > (f : F , scope : Span , tokens : TokenStream) -> Result < F :: Output > { f . __parse_scoped (scope , tokens) }
};
}
