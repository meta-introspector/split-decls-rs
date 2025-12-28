macro_rules! is_xid_start {
    () => {
        # [doc = " Whether the character has the Unicode property XID\\_Start."] pub fn is_xid_start (ch : char) -> bool { if ch . is_ascii () { return ASCII_START & (1 << ch as u128) != 0 ; } let chunk = * TRIE_START . 0 . get (ch as usize / 8 / CHUNK) . unwrap_or (& ZERO) ; let offset = chunk as usize * CHUNK / 2 + ch as usize / 8 % CHUNK ; unsafe { LEAF . 0 . get_unchecked (offset) } . wrapping_shr (ch as u32 % 8) & 1 != 0 }
    };
}

is_xid_start!()