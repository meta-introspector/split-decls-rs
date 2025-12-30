// Generated macro for impl_1486 (impl)
macro_rules! Depcrate_utilimpl_1486 {
() => {
// Module: crate::util
// Provides: {"impl_1486"}
// Dependencies: {}
# [cfg (windows)] impl OsStrExt for OsStr { fn starts_with (& self , s : & str) -> bool { let u16s = self . encode_wide () ; let mut utf8 = s . chars () ; for codepoint in u16s { let to_match = match utf8 . next () { Some (ch) => ch , None => return true , } ; let to_match = to_match as u32 ; let codepoint = codepoint as u32 ; if to_match < 0xd7ff { if to_match != codepoint { return false ; } } else { return false ; } } utf8 . next () . is_none () } fn split_prefix (& self , s : & str) -> Option < OsString > { let mut u16s = self . encode_wide () . peekable () ; let mut utf8 = s . chars () ; while let Some (& codepoint) = u16s . peek () { let to_match = match utf8 . next () { Some (ch) => ch , None => { let codepoints = u16s . collect :: < Vec < _ > > () ; return Some (OsString :: from_wide (& codepoints)) ; } } ; let to_match = to_match as u32 ; let codepoint = codepoint as u32 ; if to_match < 0xd7ff { if to_match != codepoint { return None ; } } else { return None ; } u16s . next () ; } if utf8 . next () . is_none () { Some (OsString :: new ()) } else { None } } }
};
}
