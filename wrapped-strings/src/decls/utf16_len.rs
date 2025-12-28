macro_rules! utf16_len {
    () => {
        # [doc (hidden)] pub const fn utf16_len (bytes : & [u8]) -> usize { let mut pos = 0 ; let mut len = 0 ; while let Some ((code_point , new_pos)) = decode_utf8_char (bytes , pos) { pos = new_pos ; len += if code_point <= 0xffff { 1 } else { 2 } ; } len }
    };
}

utf16_len!();