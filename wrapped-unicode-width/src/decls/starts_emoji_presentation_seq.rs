macro_rules! starts_emoji_presentation_seq {
    () => {
        # [doc = " Whether this character forms an [emoji presentation sequence]"] # [doc = " (https://www.unicode.org/reports/tr51/#def_emoji_presentation_sequence)"] # [doc = " when followed by `'\\u{FEOF}'`."] # [doc = " Emoji presentation sequences are considered to have width 2."] # [inline] pub fn starts_emoji_presentation_seq (c : char) -> bool { let cp : u32 = c . into () ; let top_bits = cp >> 10 ; let idx_of_leaf : usize = match top_bits { 0x0 => 0 , 0x8 => 1 , 0x9 => 2 , 0xA => 3 , 0xC => 4 , 0x7C => 5 , 0x7D => 6 , _ => return false , } ; let idx_within_leaf = usize :: try_from ((cp >> 3) & 0x7F) . unwrap () ; let leaf_byte = EMOJI_PRESENTATION_LEAVES . 0 [idx_of_leaf] [idx_within_leaf] ; ((leaf_byte >> (cp & 7)) & 1) == 1 }
    };
}

starts_emoji_presentation_seq!()