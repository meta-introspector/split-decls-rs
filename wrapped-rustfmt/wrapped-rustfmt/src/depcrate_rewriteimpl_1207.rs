// Generated macro for impl_1207 (impl)
macro_rules! Depcrate_rewriteimpl_1207 {
() => {
// Module: crate::rewrite
// Provides: {"impl_1207"}
// Dependencies: {}
impl < T > RewriteErrorExt < T > for Option < T > { fn max_width_error (self , width : usize , span : Span) -> Result < T , RewriteError > { self . ok_or_else (| | RewriteError :: ExceedsMaxWidth { configured_width : width , span : span , }) } fn macro_error (self , kind : MacroErrorKind , span : Span) -> Result < T , RewriteError > { self . ok_or_else (| | RewriteError :: MacroFailure { kind : kind , span : span , }) } fn unknown_error (self) -> Result < T , RewriteError > { self . ok_or_else (| | RewriteError :: Unknown) } }
};
}
