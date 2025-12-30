// Generated macro for impl_832 (impl)
macro_rules! Depcrate_displaynames_variantimpl_832 {
() => {
// Module: crate::displaynames::variant
// Provides: {"impl_832"}
// Dependencies: {}
impl TryFrom < & cldr_serde :: displaynames :: variant :: Resource > for VariantDisplayNames < 'static > { type Error = ParseError ; fn try_from (other : & cldr_serde :: displaynames :: variant :: Resource) -> Result < Self , Self :: Error > { let mut names = BTreeMap :: new () ; for entry in other . main . value . localedisplaynames . variants . iter () { if ! entry . 0 . contains (ALT_SUBSTRING) { names . insert (Variant :: try_from_str (entry . 0) ? . to_tinystr () , entry . 1 . as_str () ,) ; } } Ok (Self { names : names . into_iter () . filter (| & (k , v) | k != v) . map (| (k , v) | (k . to_unvalidated () , v)) . collect () , }) } }
};
}
