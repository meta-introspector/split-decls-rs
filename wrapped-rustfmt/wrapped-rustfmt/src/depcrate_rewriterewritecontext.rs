// Generated macro for RewriteContext (struct)
macro_rules! Depcrate_rewriteRewriteContext {
() => {
// Module: crate::rewrite
// Provides: {"RewriteContext"}
// Dependencies: {}
# [derive (Clone)] pub (crate) struct RewriteContext < 'a > { pub (crate) psess : & 'a ParseSess , pub (crate) config : & 'a Config , pub (crate) inside_macro : Rc < Cell < bool > > , pub (crate) use_block : Cell < bool > , pub (crate) is_if_else_block : Cell < bool > , pub (crate) force_one_line_chain : Cell < bool > , pub (crate) snippet_provider : & 'a SnippetProvider , pub (crate) macro_rewrite_failure : Cell < bool > , pub (crate) is_macro_def : bool , pub (crate) report : FormatReport , pub (crate) skip_context : SkipContext , pub (crate) skipped_range : Rc < RefCell < Vec < (usize , usize) > > > , }
};
}
