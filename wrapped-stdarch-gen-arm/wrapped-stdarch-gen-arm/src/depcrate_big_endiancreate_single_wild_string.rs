// Generated macro for create_single_wild_string (function)
macro_rules! Depcrate_big_endiancreate_single_wild_string {
() => {
// Module: crate::big_endian
// Provides: {"create_single_wild_string"}
// Dependencies: {}
# [doc = " Simplifies creating a string that can be used in an Expression, as Expression"] # [doc = " expects all strings to be `WildString`"] fn create_single_wild_string (name : & str) -> WildString { WildString (vec ! [WildStringPart :: String (name . to_string ())]) }
};
}
