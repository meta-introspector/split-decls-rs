// Generated macro for test_basic (function)
macro_rules! Depcrate_properties_emoji_settest_basic {
() => {
// Module: crate::properties::emoji_set
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: properties :: { props :: BasicEmoji , EmojiSetData } ; let provider = SourceDataProvider :: new_testing () ; let basic_emoji = EmojiSetData :: try_new_unstable :: < BasicEmoji > (& provider) . unwrap () ; let basic_emoji = basic_emoji . as_code_point_inversion_list_string_list () . unwrap () ; assert ! (! basic_emoji . contains32 (0x0020)) ; assert ! (! basic_emoji . contains ('\n')) ; assert ! (basic_emoji . contains ('🦃')) ; assert ! (basic_emoji . contains_str ("\u{1F983}")) ; assert ! (basic_emoji . contains_str ("\u{1F6E4}\u{FE0F}")) ; assert ! (! basic_emoji . contains_str ("\u{0033}\u{FE0F}\u{20E3}")) ; }
};
}
