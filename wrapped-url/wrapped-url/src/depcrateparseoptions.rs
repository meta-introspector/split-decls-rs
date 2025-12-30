// Generated macro for ParseOptions (struct)
macro_rules! DepcrateParseOptions {
() => {
// Module: crate
// Provides: {"ParseOptions"}
// Dependencies: {}
# [doc = " Full configuration for the URL parser."] # [derive (Copy , Clone)] # [must_use] pub struct ParseOptions < 'a > { base_url : Option < & 'a Url > , encoding_override : EncodingOverride < 'a > , violation_fn : Option < & 'a dyn Fn (SyntaxViolation) > , }
};
}
