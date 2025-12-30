// Generated macro for impl_1393 (impl)
macro_rules! Depcrate_units_helpersimpl_1393 {
() => {
// Module: crate::units::helpers
// Provides: {"impl_1393"}
// Dependencies: {}
impl Patterns { pub (crate) fn try_into_plural_elements_packed_cow (& self ,) -> Result < PluralElementsPackedCow < 'static , SinglePlaceholderPattern > , DataError > { let other_pattern = self . other . as_deref () . ok_or_else (| | { DataErrorKind :: IdentifierNotFound . into_error () . with_debug_context (self) }) ? ; Ok (PluralElements :: new (other_pattern) . with_zero_value (self . zero . as_deref ()) . with_one_value (self . one . as_deref ()) . with_two_value (self . two . as_deref ()) . with_few_value (self . few . as_deref ()) . with_many_value (self . many . as_deref ()) . with_explicit_one_value (self . explicit_one . as_deref ()) . with_explicit_zero_value (self . explicit_zero . as_deref ()) . into ()) } }
};
}
