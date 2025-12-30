// Generated macro for impl_1102 (impl)
macro_rules! Depcrate_relativetimeimpl_1102 {
() => {
// Module: crate::relativetime
// Provides: {"impl_1102"}
// Dependencies: {}
impl From < & cldr_serde :: date_fields :: PluralRulesPattern > for PluralElementsPackedCow < '_ , SinglePlaceholderPattern > { fn from (field : & cldr_serde :: date_fields :: PluralRulesPattern) -> Self { PluralElements :: new (& * field . other) . with_zero_value (field . zero . as_deref ()) . with_one_value (field . one . as_deref ()) . with_two_value (field . two . as_deref ()) . with_few_value (field . few . as_deref ()) . with_many_value (field . many . as_deref ()) . with_explicit_one_value (field . explicit_one . as_deref ()) . with_explicit_zero_value (field . explicit_zero . as_deref ()) . into () } }
};
}
