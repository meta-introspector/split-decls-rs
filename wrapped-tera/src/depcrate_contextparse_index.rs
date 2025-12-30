// Generated macro for parse_index (function)
macro_rules! Depcrate_contextparse_index {
() => {
// Module: crate::context
// Provides: {"parse_index"}
// Dependencies: {}
# [doc = " serde jsons parse_index"] # [inline] fn parse_index (s : & str) -> Option < usize > { if s . starts_with ('+') || (s . starts_with ('0') && s . len () != 1) { return None ; } s . parse () . ok () }
};
}
