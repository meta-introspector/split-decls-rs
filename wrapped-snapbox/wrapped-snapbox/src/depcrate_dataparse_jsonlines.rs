// Generated macro for parse_jsonlines (function)
macro_rules! Depcrate_dataparse_jsonlines {
() => {
// Module: crate::data
// Provides: {"parse_jsonlines"}
// Dependencies: {}
# [cfg (feature = "json")] fn parse_jsonlines (text : & str) -> Result < Vec < serde_json :: Value > , serde_json :: Error > { let mut lines = Vec :: new () ; for line in text . lines () { let line = line . trim () ; if line . is_empty () { continue ; } let json = serde_json :: from_str :: < serde_json :: Value > (line) ? ; lines . push (json) ; } Ok (lines) }
};
}
