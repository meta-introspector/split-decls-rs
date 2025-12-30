// Generated macro for tests (module)
macro_rules! Depcrate_emoji_propertiestests {
() => {
// Module: crate::emoji_properties
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: EmojiProperty ; # [test] fn parse_single () { let line = "24C2          ; Emoji                #  1.1  [1] (Ⓜ️)       circled M\n" ; let row : EmojiProperty = line . parse () . unwrap () ; assert_eq ! (row . codepoints , 0x24C2) ; assert_eq ! (row . property , "Emoji") ; } # [test] fn parse_range () { let line = "1FA6E..1FFFD  ; Extended_Pictographic#   NA[1424] (🩮️..🿽️)   <reserved-1FA6E>..<reserved-1FFFD>\n" ; let row : EmojiProperty = line . parse () . unwrap () ; assert_eq ! (row . codepoints , (0x1FA6E , 0x1FFFD)) ; assert_eq ! (row . property , "Extended_Pictographic") ; } }
};
}
