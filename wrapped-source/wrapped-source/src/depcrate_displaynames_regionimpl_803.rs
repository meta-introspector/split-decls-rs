// Generated macro for impl_803 (impl)
macro_rules! Depcrate_displaynames_regionimpl_803 {
() => {
// Module: crate::displaynames::region
// Provides: {"impl_803"}
// Dependencies: {}
impl TryFrom < & cldr_serde :: displaynames :: region :: Resource > for RegionDisplayNames < 'static > { type Error = icu :: locale :: ParseError ; fn try_from (other : & cldr_serde :: displaynames :: region :: Resource) -> Result < Self , Self :: Error > { let mut names = BTreeMap :: new () ; let mut short_names = BTreeMap :: new () ; for (region , value) in other . main . value . localedisplaynames . regions . iter () { if let Some (region) = region . strip_suffix (SHORT_SUBSTRING) { short_names . insert (Region :: try_from_str (region) ? . to_tinystr () , value . as_str ()) ; } else if ! region . contains (ALT_SUBSTRING) { names . insert (Region :: try_from_str (region) ? . to_tinystr () , value . as_str ()) ; } } Ok (Self { names : names . into_iter () . filter (| & (k , v) | k != v) . map (| (k , v) | (k . to_unvalidated () , v)) . collect () , short_names : short_names . into_iter () . filter (| & (k , v) | k != v) . map (| (k , v) | (k . to_unvalidated () , v)) . collect () , }) } }
};
}
