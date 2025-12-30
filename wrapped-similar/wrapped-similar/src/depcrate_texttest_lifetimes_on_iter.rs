// Generated macro for test_lifetimes_on_iter (function)
macro_rules! Depcrate_texttest_lifetimes_on_iter {
() => {
// Module: crate::text
// Provides: {"test_lifetimes_on_iter"}
// Dependencies: {}
# [test] fn test_lifetimes_on_iter () { use crate :: Change ; fn diff_lines < 'x , T > (old : & 'x T , new : & 'x T) -> Vec < Change < & 'x T :: Output > > where T : DiffableStrRef + ? Sized , { TextDiff :: from_lines (old , new) . iter_all_changes () . collect () } let a = "1\n2\n3\n" . to_string () ; let b = "1\n99\n3\n" . to_string () ; let changes = diff_lines (& a , & b) ; insta :: assert_debug_snapshot ! (& changes) ; }
};
}
