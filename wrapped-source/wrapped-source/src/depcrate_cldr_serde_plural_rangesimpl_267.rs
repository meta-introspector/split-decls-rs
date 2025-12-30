// Generated macro for impl_267 (impl)
macro_rules! Depcrate_cldr_serde_plural_rangesimpl_267 {
() => {
// Module: crate::cldr_serde::plural_ranges
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for PluralRange { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { struct PluralRangeVisitor ; impl Visitor < '_ > for PluralRangeVisitor { type Value = PluralRange ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (formatter , "a plural range rule of the form pluralRange-start-<plural category>-end-<plural category>" ,) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { let v = v . strip_prefix ("pluralRange-start-") . ok_or_else (| | { E :: custom ("expected prefix `pluralRange-start-` before start category") }) ? ; let (start , v) = v . split_once ('-') . ok_or_else (| | { E :: custom ("missing token `-` between start and end categories") }) ? ; let end = v . strip_prefix ("end-") . ok_or_else (| | E :: custom ("expected prefix `end-` before end category")) ? ; Ok (PluralRange { start : start . into () , end : end . into () , }) } } deserializer . deserialize_string (PluralRangeVisitor) } }
};
}
