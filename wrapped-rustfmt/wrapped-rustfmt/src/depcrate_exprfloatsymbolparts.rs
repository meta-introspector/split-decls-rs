// Generated macro for FloatSymbolParts (struct)
macro_rules! Depcrate_exprFloatSymbolParts {
() => {
// Module: crate::expr
// Provides: {"FloatSymbolParts"}
// Dependencies: {}
# [doc = " Indicates the parts of a float literal specified as a string."] struct FloatSymbolParts < 'a > { # [doc = " The integer part, e.g. `123` in `123.456e789`."] # [doc = " Always non-empty, because in Rust `.1` is not a valid floating-point literal:"] # [doc = " <https://doc.rust-lang.org/reference/tokens.html#floating-point-literals>"] integer_part : & 'a str , # [doc = " The fractional part excluding the decimal point, e.g. `456` in `123.456e789`."] fractional_part : Option < & 'a str > , # [doc = " The exponent part including the `e` or `E`, e.g. `e789` in `123.456e789`."] exponent : Option < & 'a str > , }
};
}
