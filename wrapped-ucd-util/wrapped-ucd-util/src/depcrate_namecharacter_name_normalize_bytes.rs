// Generated macro for character_name_normalize_bytes (function)
macro_rules! Depcrate_namecharacter_name_normalize_bytes {
() => {
// Module: crate::name
// Provides: {"character_name_normalize_bytes"}
// Dependencies: {}
# [doc = " Normalize the given character name in place according to UAX44-LM2."] # [doc = ""] # [doc = " The slice returned is guaranteed to be valid UTF-8 for all possible values"] # [doc = " of `slice`."] # [doc = ""] # [doc = " See: https://unicode.org/reports/tr44/#UAX44-LM2"] fn character_name_normalize_bytes (slice : & mut [u8]) -> & mut [u8] { let mut next_write = 0 ; let mut prev_letter = false ; for i in 0 .. slice . len () { let b = slice [i] ; if b == b' ' { } else if b == b'_' { } else if b == b'-' { let medial = prev_letter && slice . get (i + 1) . map_or (false , | b | b . is_ascii_alphabetic ()) ; let mut keep_hyphen = ! medial ; let next_e = slice . get (i + 1) . map_or (false , | & b | b == b'E' || b == b'e') ; let rest_empty = i + 2 >= slice . len () || slice [i + 2 ..] . iter () . all (| & b | b == b' ' || b == b'_') ; if ! keep_hyphen && next_e && rest_empty { keep_hyphen = slice [.. next_write] == b"hanguljungseongo" [..] ; } if keep_hyphen { slice [next_write] = b ; next_write += 1 ; } } else if b'A' <= b && b <= b'Z' { slice [next_write] = b + (b'a' - b'A') ; next_write += 1 ; } else if b <= 0x7F { slice [next_write] = b ; next_write += 1 ; } prev_letter = b . is_ascii_alphabetic () ; } & mut slice [.. next_write] }
};
}
