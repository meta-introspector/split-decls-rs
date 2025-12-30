// Generated macro for impl_229 (impl)
macro_rules! Depcrate_lineimpl_229 {
() => {
// Module: crate::line
// Provides: {"impl_229"}
// Dependencies: {}
impl From < LineBreakOptions < '_ > > for ResolvedLineBreakOptions { fn from (options : LineBreakOptions < '_ >) -> Self { let ja_zh = if let Some (content_locale) = options . content_locale . as_ref () { content_locale . language == language ! ("ja") || content_locale . language == language ! ("zh") } else { false } ; Self { strictness : options . strictness . unwrap_or_default () , word_option : options . word_option . unwrap_or_default () , ja_zh , } } }
};
}
