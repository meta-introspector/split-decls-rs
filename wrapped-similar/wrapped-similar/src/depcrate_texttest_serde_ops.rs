// Generated macro for test_serde_ops (function)
macro_rules! Depcrate_texttest_serde_ops {
() => {
// Module: crate::text
// Provides: {"test_serde_ops"}
// Dependencies: {}
# [test] # [cfg (feature = "serde")] fn test_serde_ops () { let diff = TextDiff :: from_lines ("Hello World\nsome stuff here\nsome more stuff here\n\nAha stuff here\nand more stuff" , "Stuff\nHello World\nsome amazing stuff here\nsome more stuff here\n" ,) ; let changes = diff . ops () ; let json = serde_json :: to_string_pretty (& changes) . unwrap () ; insta :: assert_snapshot ! (& json) ; }
};
}
