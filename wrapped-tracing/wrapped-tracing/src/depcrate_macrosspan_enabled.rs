// Generated macro for span_enabled (macro)
macro_rules! Depcrate_macrosspan_enabled {
() => {
// Module: crate::macros
// Provides: {"span_enabled"}
// Dependencies: {}
# [doc = " Tests whether a span with the specified level and target would be enabled."] # [doc = ""] # [doc = " This is similar to [`enabled!`], but queries the current subscriber specifically for"] # [doc = " an event, whereas [`enabled!`] queries for an event _or_ span."] # [doc = ""] # [doc = " See the documentation for [`enabled!]` for more details on using this macro."] # [doc = " See also [`span_enabled!`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use tracing::{span_enabled, Level};"] # [doc = " if span_enabled!(target: \"my_crate\", Level::DEBUG) {"] # [doc = "     // some expensive work..."] # [doc = " }"] # [doc = " // simpler"] # [doc = " if span_enabled!(Level::DEBUG) {"] # [doc = "     // some expensive work..."] # [doc = " }"] # [doc = " // with fields"] # [doc = " if span_enabled!(Level::DEBUG, foo_field) {"] # [doc = "     // some expensive work..."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`enabled!`]: crate::enabled"] # [doc = " [`span_enabled!`]: crate::span_enabled"] # [macro_export] macro_rules ! span_enabled { ($ ($ rest : tt) *) => ($ crate :: enabled ! (kind : $ crate :: metadata :: Kind :: SPAN , $ ($ rest) *)) }
};
}
