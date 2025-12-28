macro_rules! is_emoji {
    () => {
        fn is_emoji (ch : char) -> bool { use crate :: tables :: emoji ; emoji :: emoji_category (ch) . 2 == emoji :: EmojiCat :: EC_Extended_Pictographic }
    };
}

is_emoji!();