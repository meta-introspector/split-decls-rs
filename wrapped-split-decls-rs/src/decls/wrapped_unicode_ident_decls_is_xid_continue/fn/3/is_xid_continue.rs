use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Whether the character has the Unicode property XID\\_Continue."] pub fn is_xid_continue (ch : char) -> bool { if ch . is_ascii () { return ASCII_CONTINUE & (1 << ch as u128) != 0 ; } let chunk = * TRIE_CONTINUE . 0 . get (ch as usize / 8 / CHUNK) . unwrap_or (& ZERO) ; let offset = chunk as usize * CHUNK / 2 + ch as usize / 8 % CHUNK ; unsafe { LEAF . 0 . get_unchecked (offset) } . wrapping_shr (ch as u32 % 8) & 1 != 0 }