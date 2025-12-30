// Generated macro for to_valid_ident (function)
macro_rules! Depcrate_syn_utilsto_valid_ident {
() => {
// Module: crate::syn_utils
// Provides: {"to_valid_ident"}
// Dependencies: {}
pub fn to_valid_ident (s : & str) -> Result < Ident > { if let Ok (ident) = parse_str (s) { Ok (ident) } else { parse_str (& format ! ("r#{s}")) } }
};
}
