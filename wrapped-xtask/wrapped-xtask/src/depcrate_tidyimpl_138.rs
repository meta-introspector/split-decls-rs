// Generated macro for impl_138 (impl)
macro_rules! Depcrate_tidyimpl_138 {
() => {
// Module: crate::tidy
// Provides: {"impl_138"}
// Dependencies: {}
impl TidyDocs { fn visit (& mut self , path : & Path , text : & str) { if is_exclude_dir (path , & ["tests" , "test_data" , "fixes" , "grammar" , "stdx"]) { return ; } if is_exclude_file (path) { return ; } let first_line = match text . lines () . next () { Some (it) => it , None => return , } ; if first_line . starts_with ("//!") { if first_line . contains ("FIXME") { self . contains_fixme . push (path . to_path_buf ()) ; } } else { if text . contains ("// Feature:") || text . contains ("// Assist:") || text . contains ("// Diagnostic:") { return ; } self . missing_docs . push (path . display () . to_string ()) ; } fn is_exclude_file (d : & Path) -> bool { let file_names = ["tests.rs" , "famous_defs_fixture.rs"] ; d . file_name () . unwrap_or_default () . to_str () . map (| f_n | file_names . contains (& f_n)) . unwrap_or (false) } } fn finish (self) { if ! self . missing_docs . is_empty () { panic ! ("\nMissing docs strings\n\n\
                 modules:\n{}\n\n" , self . missing_docs . join ("\n")) } if let Some (path) = self . contains_fixme . first () { panic ! ("FIXME doc in a fully-documented crate: {}" , path . display ()) } } }
};
}
