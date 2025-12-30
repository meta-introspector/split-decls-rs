// Generated macro for tests (module)
macro_rules! Depcrate_struct_metatests {
() => {
// Module: crate::struct_meta
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_is_option () { assert_eq ! (get_option_element (& parse_quote ! (Option < u8 >)) , Some (& parse_quote ! (u8))) ; } # [test] fn test_is_option_mod () { assert_eq ! (get_option_element (& parse_quote ! (option :: Option < u8 >)) , Some (& parse_quote ! (u8))) ; } # [test] fn test_is_option_core () { assert_eq ! (get_option_element (& parse_quote ! (core :: option :: Option < u8 >)) , Some (& parse_quote ! (u8))) ; } # [test] fn test_is_option_std () { assert_eq ! (get_option_element (& parse_quote ! (std :: option :: Option < u8 >)) , Some (& parse_quote ! (u8))) ; } }
};
}
