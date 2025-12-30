// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_rewriteimpl_1225 {
() => {
// Module: crate::rewrite
// Provides: {"impl_1225"}
// Dependencies: {}
impl From < ExceedsMaxWidthError > for RewriteError { fn from (error : ExceedsMaxWidthError) -> Self { RewriteError :: ExceedsMaxWidth { configured_width : error . configured_width , span : error . span , } } }
};
}
