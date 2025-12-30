// Generated macro for impl_329 (impl)
macro_rules! Depcrate_unicode_dataimpl_329 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_329"}
// Dependencies: {}
impl std :: str :: FromStr for UnicodeDataDecomposition { type Err = Error ; fn from_str (s : & str) -> Result < UnicodeDataDecomposition , Error > { let re_with_tag = regex ! (r"^(?:<(?P<tag>[^>]+)>)?\s*(?P<chars>[\s0-9A-F]+)$") ; let re_chars = regex ! (r"[0-9A-F]+") ; if s . is_empty () { return err ! ("expected non-empty string for \
                 UnicodeDataDecomposition value") ; } let caps = match re_with_tag . captures (s) { Some (caps) => caps , None => return err ! ("invalid decomposition value") , } ; let mut decomp = UnicodeDataDecomposition :: default () ; let mut codepoints = s ; if let Some (m) = caps . name ("tag") { decomp . tag = Some (m . as_str () . parse () ?) ; codepoints = & caps ["chars"] ; } for m in re_chars . find_iter (codepoints) { let cp = m . as_str () . parse () ? ; decomp . push (cp) ? ; } Ok (decomp) } }
};
}
