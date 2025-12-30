// Generated macro for impl_1005 (impl)
macro_rules! Depcrate_pluralsimpl_1005 {
() => {
// Module: crate::plurals
// Provides: {"impl_1005"}
// Dependencies: {}
impl From < & cldr_serde :: plurals :: LocalePluralRules > for PluralRulesData < 'static > { fn from (other : & cldr_serde :: plurals :: LocalePluralRules) -> Self { # [doc = " Removes samples from plural rule strings. Takes an owned [`String`] reference and"] # [doc = " returns a new [`String`] in a [`Cow::Owned`]."] fn convert (s : & str) -> Rule < 'static > { s . parse () . expect ("Rule parsing failed.") } Self { zero : other . zero . as_deref () . map (convert) , one : other . one . as_deref () . map (convert) , two : other . two . as_deref () . map (convert) , few : other . few . as_deref () . map (convert) , many : other . many . as_deref () . map (convert) , } } }
};
}
