// Generated macro for State (struct)
macro_rules! Depcrate_parser_inline_tableState {
() => {
// Module: crate::parser::inline_table
// Provides: {"State"}
// Dependencies: {}
# [derive (Default)] struct State { current_prefix : Option < toml_parser :: Span > , current_key : Option < (Vec < Key > , Key) > , seen_keyval_sep : bool , current_value : Option < Value > , current_suffix : Option < toml_parser :: Span > , }
};
}
