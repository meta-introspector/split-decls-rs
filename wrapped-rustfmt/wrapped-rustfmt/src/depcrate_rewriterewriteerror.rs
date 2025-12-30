// Generated macro for RewriteError (enum)
macro_rules! Depcrate_rewriteRewriteError {
() => {
// Module: crate::rewrite
// Provides: {"RewriteError"}
// Dependencies: {}
# [derive (Clone , Error , Debug)] pub (crate) enum RewriteError { # [error ("Formatting was skipped due to skip attribute or out of file range.")] SkipFormatting , # [error ("It exceeds the required width of {configured_width} for the span: {span:?}")] ExceedsMaxWidth { configured_width : usize , span : Span } , # [error ("Failed to format given macro{} at: {span:?}" , kind)] MacroFailure { kind : MacroErrorKind , span : Span } , # [doc = " Format failure that does not fit to above categories."] # [error ("An unknown error occurred during formatting.")] Unknown , }
};
}
