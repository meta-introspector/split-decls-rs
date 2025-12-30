// Generated macro for FmtVisitor (struct)
macro_rules! Depcrate_modules_visitorFmtVisitor {
() => {
// Module: crate::modules::visitor
// Provides: {"FmtVisitor"}
// Dependencies: {}
pub (crate) struct FmtVisitor < 'a > { parent_context : Option < & 'a RewriteContext < 'a > > , pub (crate) psess : & 'a ParseSess , pub (crate) buffer : String , pub (crate) last_pos : BytePos , pub (crate) block_indent : Indent , pub (crate) config : & 'a Config , pub (crate) is_if_else_block : bool , pub (crate) snippet_provider : & 'a SnippetProvider , pub (crate) line_number : usize , # [doc = " List of 1-based line ranges which were annotated with skip"] # [doc = " Both bounds are inclusive."] pub (crate) skipped_range : Rc < RefCell < Vec < (usize , usize) > > > , pub (crate) macro_rewrite_failure : bool , pub (crate) report : FormatReport , pub (crate) skip_context : SkipContext , pub (crate) is_macro_def : bool , }
};
}
