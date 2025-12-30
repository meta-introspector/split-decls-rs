// Generated macro for string_to_box_u8 (function)
macro_rules! Depcrate_zerotriestring_to_box_u8 {
() => {
// Module: crate::zerotrie
// Provides: {"string_to_box_u8"}
// Dependencies: {}
# [cfg (feature = "alloc")] fn string_to_box_u8 (input : String) -> Box < [u8] > { input . into_boxed_str () . into_boxed_bytes () }
};
}
