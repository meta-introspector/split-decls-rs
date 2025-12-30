// Generated macro for FieldSet (struct)
macro_rules! Depcrate_fieldFieldSet {
() => {
// Module: crate::field
// Provides: {"FieldSet"}
// Dependencies: {}
# [doc = " Describes the fields present on a span."] # [doc = ""] # [doc = " ## Equality"] # [doc = ""] # [doc = " In well-behaved applications, two `FieldSet`s [initialized] with equal"] # [doc = " [callsite identifiers] will have identical fields. Consequently, in release"] # [doc = " builds, [`FieldSet::eq`] *only* checks that its arguments have equal"] # [doc = " callsites. However, the equality of field names is checked in debug builds."] # [doc = ""] # [doc = " [initialized]: Self::new"] # [doc = " [callsite identifiers]: callsite::Identifier"] pub struct FieldSet { # [doc = " The names of each field on the described span."] names : & 'static [& 'static str] , # [doc = " The callsite where the described span originates."] callsite : callsite :: Identifier , }
};
}
