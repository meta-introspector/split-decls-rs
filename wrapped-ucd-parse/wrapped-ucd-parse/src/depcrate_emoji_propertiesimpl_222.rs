// Generated macro for impl_222 (impl)
macro_rules! Depcrate_emoji_propertiesimpl_222 {
() => {
// Module: crate::emoji_properties
// Provides: {"impl_222"}
// Dependencies: {}
impl std :: str :: FromStr for EmojiProperty { type Err = Error ; fn from_str (line : & str) -> Result < EmojiProperty , Error > { let (codepoints , property) = parse_codepoint_association (line) ? ; Ok (EmojiProperty { codepoints , property : property . to_string () }) } }
};
}
