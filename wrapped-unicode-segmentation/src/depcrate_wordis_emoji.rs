// Generated macro for is_emoji (function)
macro_rules! Depcrate_wordis_emoji {
() => {
// Module: crate::word
// Provides: {"is_emoji"}
// Dependencies: {}
fn is_emoji (ch : char) -> bool { use crate :: tables :: emoji ; emoji :: emoji_category (ch) . 2 == emoji :: EmojiCat :: EC_Extended_Pictographic }
};
}
