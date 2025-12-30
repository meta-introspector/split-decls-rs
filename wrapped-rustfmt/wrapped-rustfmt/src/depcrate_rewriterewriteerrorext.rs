// Generated macro for RewriteErrorExt (trait)
macro_rules! Depcrate_rewriteRewriteErrorExt {
() => {
// Module: crate::rewrite
// Provides: {"RewriteErrorExt"}
// Dependencies: {}
# [doc = " Extension trait used to conveniently convert to RewriteError"] pub (crate) trait RewriteErrorExt < T > { fn max_width_error (self , width : usize , span : Span) -> Result < T , RewriteError > ; fn macro_error (self , kind : MacroErrorKind , span : Span) -> Result < T , RewriteError > ; fn unknown_error (self) -> Result < T , RewriteError > ; }
};
}
