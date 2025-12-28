macro_rules! char_to_utf8 {
    () => {
        # [inline] fn char_to_utf8 (c : char , dst : & mut [u8 ; 4]) -> usize { const TAG_CONT : u8 = 0b1000_0000 ; const TAG_TWO_B : u8 = 0b1100_0000 ; const TAG_THREE_B : u8 = 0b1110_0000 ; const TAG_FOUR_B : u8 = 0b1111_0000 ; let code = c as u32 ; if code <= 0x7F { dst [0] = code as u8 ; 1 } else if code <= 0x7FF { dst [0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B ; dst [1] = (code & 0x3F) as u8 | TAG_CONT ; 2 } else if code <= 0xFFFF { dst [0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B ; dst [1] = (code >> 6 & 0x3F) as u8 | TAG_CONT ; dst [2] = (code & 0x3F) as u8 | TAG_CONT ; 3 } else { dst [0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B ; dst [1] = (code >> 12 & 0x3F) as u8 | TAG_CONT ; dst [2] = (code >> 6 & 0x3F) as u8 | TAG_CONT ; dst [3] = (code & 0x3F) as u8 | TAG_CONT ; 4 } }
    };
}

char_to_utf8!();