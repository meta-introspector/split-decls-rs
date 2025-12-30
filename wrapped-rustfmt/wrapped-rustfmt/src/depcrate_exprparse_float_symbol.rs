// Generated macro for parse_float_symbol (function)
macro_rules! Depcrate_exprparse_float_symbol {
() => {
// Module: crate::expr
// Provides: {"parse_float_symbol"}
// Dependencies: {}
# [doc = " Parses a float literal. The `symbol` must be a valid floating point literal without a type"] # [doc = " suffix. Otherwise the function may panic or return wrong result."] fn parse_float_symbol (symbol : & str) -> Result < FloatSymbolParts < '_ > , & 'static str > { let float_literal_regex = static_regex ! (r"^([0-9_]+)(?:\.([0-9_]+)?)?([eE][+-]?[0-9_]+)?$") ; let caps = float_literal_regex . captures (symbol) . ok_or ("invalid float literal") ? ; Ok (FloatSymbolParts { integer_part : caps . get (1) . ok_or ("missing integer part") ? . as_str () , fractional_part : caps . get (2) . map (| m | m . as_str ()) , exponent : caps . get (3) . map (| m | m . as_str ()) , }) }
};
}
