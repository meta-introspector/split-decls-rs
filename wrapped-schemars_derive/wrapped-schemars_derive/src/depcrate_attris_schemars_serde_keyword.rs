// Generated macro for is_schemars_serde_keyword (function)
macro_rules! Depcrate_attris_schemars_serde_keyword {
() => {
// Module: crate::attr
// Provides: {"is_schemars_serde_keyword"}
// Dependencies: {}
fn is_schemars_serde_keyword (meta : & CustomMeta) -> bool { let known_keywords = schemars_to_serde :: SCHEMARS_KEYWORDS_PARSED_BY_SERDE ; meta . path () . get_ident () . is_some_and (| i | known_keywords . contains (& i . to_string () . as_str ())) }
};
}
