// Generated macro for impl_922 (impl)
macro_rules! Depcrate_locale_parentsimpl_922 {
() => {
// Module: crate::locale::parents
// Provides: {"impl_922"}
// Dependencies: {}
impl From < & cldr_serde :: parent_locales :: Resource > for Parents < 'static > { fn from (source_data : & cldr_serde :: parent_locales :: Resource) -> Self { let mut parents = BTreeMap :: < _ , (Language , Option < Script > , Option < Region >) > :: new () ; for (source , target) in source_data . supplemental . parent_locales . parent_locale . iter () { assert ! (! source . language . is_unknown ()) ; if source . script . is_some () && source . region . is_none () && target . is_unknown () { continue ; } parents . insert (source . write_to_string () , target . into ()) ; } parents . insert ("und-Hant" . into () , (Language :: UNKNOWN , Some (script ! ("Hani")) , None) ,) ; parents . insert ("und-Hans" . into () , (Language :: UNKNOWN , Some (script ! ("Hani")) , None) ,) ; Parents { parents : parents . iter () . map (| (k , v) | (< & PotentialUtf8 > :: from (k . as_ref ()) , v)) . collect () , } } }
};
}
