// Generated macro for impl_818 (impl)
macro_rules! Depcrate_displaynames_scriptimpl_818 {
() => {
// Module: crate::displaynames::script
// Provides: {"impl_818"}
// Dependencies: {}
impl TryFrom < & cldr_serde :: displaynames :: script :: Resource > for ScriptDisplayNames < 'static > { type Error = ParseError ; fn try_from (other : & cldr_serde :: displaynames :: script :: Resource) -> Result < Self , Self :: Error > { let mut names = BTreeMap :: new () ; let mut short_names = BTreeMap :: new () ; for entry in other . main . value . localedisplaynames . scripts . iter () { if let Some (script) = entry . 0 . strip_suffix (ALT_SHORT_SUBSTRING) { short_names . insert (Script :: try_from_str (script) ? . to_tinystr () , entry . 1 . as_str ()) ; } else if ! entry . 0 . contains (ALT_SUBSTRING) { names . insert (Script :: try_from_str (entry . 0) ? . to_tinystr () , entry . 1 . as_str () ,) ; } } Ok (Self { names : names . into_iter () . filter (| & (k , v) | k != v) . map (| (k , v) | (k . to_unvalidated () , v)) . collect () , short_names : short_names . into_iter () . filter (| & (k , v) | k != v) . map (| (k , v) | (k . to_unvalidated () , v)) . collect () , }) } }
};
}
