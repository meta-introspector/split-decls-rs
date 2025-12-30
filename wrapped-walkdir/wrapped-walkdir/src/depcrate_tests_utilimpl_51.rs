// Generated macro for impl_51 (impl)
macro_rules! Depcrate_tests_utilimpl_51 {
() => {
// Module: crate::tests::util
// Provides: {"impl_51"}
// Dependencies: {}
impl RecursiveResults { # [doc = " Return all of the errors encountered during traversal."] pub fn errs (& self) -> & [Error] { & self . errs } # [doc = " Assert that no errors have occurred."] pub fn assert_no_errors (& self) { assert ! (self . errs . is_empty () , "expected to find no errors, but found: {:?}" , self . errs) ; } # [doc = " Return all the successfully retrieved directory entries in the order"] # [doc = " in which they were retrieved."] pub fn ents (& self) -> & [DirEntry] { & self . ents } # [doc = " Return all paths from all successfully retrieved directory entries."] # [doc = ""] # [doc = " This does not include paths that correspond to an error."] pub fn paths (& self) -> Vec < PathBuf > { self . ents . iter () . map (| d | d . path () . to_path_buf ()) . collect () } # [doc = " Return all the successfully retrieved directory entries, sorted"] # [doc = " lexicographically by their full file path."] pub fn sorted_ents (& self) -> Vec < DirEntry > { let mut ents = self . ents . clone () ; ents . sort_by (| e1 , e2 | e1 . path () . cmp (e2 . path ())) ; ents } # [doc = " Return all paths from all successfully retrieved directory entries,"] # [doc = " sorted lexicographically."] # [doc = ""] # [doc = " This does not include paths that correspond to an error."] pub fn sorted_paths (& self) -> Vec < PathBuf > { self . sorted_ents () . into_iter () . map (| d | d . into_path ()) . collect () } }
};
}
