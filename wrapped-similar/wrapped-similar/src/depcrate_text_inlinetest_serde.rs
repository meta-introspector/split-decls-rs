// Generated macro for test_serde (function)
macro_rules! Depcrate_text_inlinetest_serde {
() => {
// Module: crate::text::inline
// Provides: {"test_serde"}
// Dependencies: {}
# [test] # [cfg (feature = "serde")] fn test_serde () { let diff = TextDiff :: from_lines ("Hello World\nsome stuff here\nsome more stuff here\n\nAha stuff here\nand more stuff" , "Stuff\nHello World\nsome amazing stuff here\nsome more stuff here\n" ,) ; assert ! (diff . newline_terminated ()) ; let changes = diff . ops () . iter () . flat_map (| op | diff . iter_inline_changes (op)) . collect :: < Vec < _ > > () ; let json = serde_json :: to_string_pretty (& changes) . unwrap () ; insta :: assert_snapshot ! (& json) ; }
};
}
