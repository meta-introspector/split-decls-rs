// Generated macro for make_alphabet (function)
macro_rules! Depcratemake_alphabet {
() => {
// Module: crate
// Provides: {"make_alphabet"}
// Dependencies: {}
fn make_alphabet < I > (enc : & 'static Encoding , codepoints : I) -> String where I : IntoIterator < Item = char > , { let iter = codepoints . into_iter () ; let mut alphabet = String :: with_capacity (iter . size_hint () . 1 . unwrap_or (256) * 4) ; for ch in '\u{0000}' ..= '\u{007F}' { if is_literal_xml11_char (ch) { alphabet . push (ch) ; } } for (pointer , cp) in iter . enumerate () { if enc == BIG5 && pointer < 5024 { continue ; } if enc == SHIFT_JIS && (8272 ..= 8835) . contains (& pointer) { continue ; } if is_literal_xml11_char (cp) { alphabet . push (cp) ; } } alphabet }
};
}
