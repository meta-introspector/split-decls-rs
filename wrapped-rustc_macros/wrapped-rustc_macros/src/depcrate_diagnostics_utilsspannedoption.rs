// Generated macro for SpannedOption (type)
macro_rules! Depcrate_diagnostics_utilsSpannedOption {
() => {
// Module: crate::diagnostics::utils
// Provides: {"SpannedOption"}
// Dependencies: {}
# [doc = " An [`Option<T>`] that keeps track of the span that caused it to be set; used with [`SetOnce`]."] pub (super) type SpannedOption < T > = Option < (T , Span) > ;
};
}
