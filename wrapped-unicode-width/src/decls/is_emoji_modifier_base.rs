macro_rules! is_emoji_modifier_base {
    () => {
        # [doc = " Returns `true` if `c` is an `Emoji_Modifier_Base`."] # [inline] pub fn is_emoji_modifier_base (c : char) -> bool { let cp : u32 = c . into () ; let top_bits = cp >> 8 ; let leaf : & [(u8 , u8)] = match top_bits { 0x26 => & EMOJI_MODIFIER_LEAF_0 , 0x27 => & EMOJI_MODIFIER_LEAF_1 , 0x1F3 => & EMOJI_MODIFIER_LEAF_2 , 0x1F4 => & EMOJI_MODIFIER_LEAF_3 , 0x1F5 => & EMOJI_MODIFIER_LEAF_4 , 0x1F6 => & EMOJI_MODIFIER_LEAF_5 , 0x1F9 => & EMOJI_MODIFIER_LEAF_6 , 0x1FA => & EMOJI_MODIFIER_LEAF_7 , _ => return false , } ; let bottom_bits = (cp & 0xFF) as u8 ; leaf . binary_search_by (| & (lo , hi) | { if bottom_bits < lo { Ordering :: Greater } else if bottom_bits > hi { Ordering :: Less } else { Ordering :: Equal } }) . is_ok () }
    };
}

is_emoji_modifier_base!();