// Generated macro for EmojiProperty (struct)
macro_rules! Depcrate_emoji_propertiesEmojiProperty {
() => {
// Module: crate::emoji_properties
// Provides: {"EmojiProperty"}
// Dependencies: {}
# [doc = " A single row in the `emoji-data.txt` file."] # [doc = ""] # [doc = " The `emoji-data.txt` file is the source of truth on several Emoji-related"] # [doc = " Unicode properties."] # [doc = ""] # [doc = " Note that `emoji-data.txt` is not formally part of the Unicode Character"] # [doc = " Database. You can download the Emoji data files separately here:"] # [doc = " https://unicode.org/Public/emoji/"] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct EmojiProperty { # [doc = " The codepoint or codepoint range for this entry."] pub codepoints : Codepoints , # [doc = " The property name assigned to the codepoints in this entry."] pub property : String , }
};
}
