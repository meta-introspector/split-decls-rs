// Generated macro for impl_887 (impl)
macro_rules! Depcrate_locale_directionalityimpl_887 {
() => {
// Module: crate::locale::directionality
// Provides: {"impl_887"}
// Dependencies: {}
impl From < & cldr_serde :: directionality :: Resource > for ScriptDirection < '_ > { fn from (other : & cldr_serde :: directionality :: Resource) -> Self { let mut rtl = vec ! [] ; let mut ltr = vec ! [] ; for (script , metadata) in & other . script_metadata { match metadata . rtl { cldr_serde :: directionality :: Rtl :: Yes => rtl . push (script . to_unvalidated ()) , cldr_serde :: directionality :: Rtl :: No => ltr . push (script . to_unvalidated ()) , cldr_serde :: directionality :: Rtl :: Unknown => () , } } rtl . sort_unstable () ; ltr . sort_unstable () ; Self { rtl : rtl . into_iter () . collect () , ltr : ltr . into_iter () . collect () , } } }
};
}
