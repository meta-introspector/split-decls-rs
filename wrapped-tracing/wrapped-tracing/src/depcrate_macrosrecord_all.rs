// Generated macro for record_all (macro)
macro_rules! Depcrate_macrosrecord_all {
() => {
// Module: crate::macros
// Provides: {"record_all"}
// Dependencies: {}
# [doc = " Records multiple values on a span in a single call. As with recording"] # [doc = " individual values, all fields must be declared when the span is created."] # [doc = ""] # [doc = " This macro supports two optional sigils:"] # [doc = " - `%` uses the Display implementation."] # [doc = " - `?` uses the Debug implementation."] # [doc = ""] # [doc = " For more details, see the [top-level documentation][lib]."] # [doc = ""] # [doc = " [lib]: tracing/#recording-fields"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use tracing::{field, info_span, record_all};"] # [doc = " let span = info_span!(\"my span\", field1 = field::Empty, field2 = field::Empty, field3 = field::Empty).entered();"] # [doc = " record_all!(span, field1 = ?\"1\", field2 = %\"2\", field3 = 3);"] # [doc = " ```"] # [macro_export] macro_rules ! record_all { ($ span : expr , $ ($ fields : tt) *) => { if let Some (meta) = $ span . metadata () { $ span . record_all (&$ crate :: valueset ! (meta . fields () , $ ($ fields) *)) ; } } ; }
};
}
