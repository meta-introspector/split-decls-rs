// Generated macro for Field (struct)
macro_rules! Depcrate_fieldField {
() => {
// Module: crate::field
// Provides: {"Field"}
// Dependencies: {}
# [doc = " An opaque key allowing _O_(1) access to a field in a `Span`'s key-value"] # [doc = " data."] # [doc = ""] # [doc = " As keys are defined by the _metadata_ of a span, rather than by an"] # [doc = " individual instance of a span, a key may be used to access the same field"] # [doc = " across all instances of a given span with the same metadata. Thus, when a"] # [doc = " subscriber observes a new span, it need only access a field by name _once_,"] # [doc = " and use the key for that name for all other accesses."] # [derive (Debug)] pub struct Field { i : usize , fields : FieldSet , }
};
}
