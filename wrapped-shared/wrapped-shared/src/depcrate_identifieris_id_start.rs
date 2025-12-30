// Generated macro for is_id_start (function)
macro_rules! Depcrate_identifieris_id_start {
() => {
// Module: crate::identifier
// Provides: {"is_id_start"}
// Dependencies: {}
# [doc = " Returns whether a character is a valid JS identifier start character."] # [doc = ""] # [doc = " This is only ever-so-slightly different from `XID_Start` in a few edge"] # [doc = " cases, so we handle those edge cases manually and delegate everything else"] # [doc = " to `unicode-ident`."] fn is_id_start (c : char) -> bool { match c { '\u{037A}' | '\u{0E33}' | '\u{0EB3}' | '\u{309B}' | '\u{309C}' | '\u{FC5E}' | '\u{FC5F}' | '\u{FC60}' | '\u{FC61}' | '\u{FC62}' | '\u{FC63}' | '\u{FDFA}' | '\u{FDFB}' | '\u{FE70}' | '\u{FE72}' | '\u{FE74}' | '\u{FE76}' | '\u{FE78}' | '\u{FE7A}' | '\u{FE7C}' | '\u{FE7E}' | '\u{FF9E}' | '\u{FF9F}' => true , '$' | '_' => true , _ => unicode_ident :: is_xid_start (c) , } }
};
}
