// Generated macro for Label (struct)
macro_rules! Depcrate_dataLabel {
() => {
// Module: crate::data
// Provides: {"Label"}
// Dependencies: {}
# [doc = "\nA textual label for some value.\n"] pub struct Label < 'computed > { value_computed : * const str , backing_field_static : Option < & 'static str > , # [cfg (feature = "alloc")] backing_field_owned : Option < * mut str > , tag : Option < Tag > , _marker : PhantomData < & 'computed str > , }
};
}
