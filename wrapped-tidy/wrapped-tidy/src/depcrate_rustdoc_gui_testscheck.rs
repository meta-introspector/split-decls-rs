// Generated macro for check (function)
macro_rules! Depcrate_rustdoc_gui_testscheck {
() => {
// Module: crate::rustdoc_gui_tests
// Provides: {"check"}
// Dependencies: {}
pub fn check (path : & Path , bad : & mut bool) { crate :: walk :: walk (& path . join ("rustdoc-gui") , | p , is_dir | ! is_dir && p . extension () . is_none_or (| e | e != "goml") , & mut | entry , content | { for line in content . lines () { if ! line . starts_with ("// ") { tidy_error ! (bad , "{}: rustdoc-gui tests must start with a small description" , entry . path () . display () ,) ; return ; } else if line . starts_with ("// ") { let parts = line [2 ..] . trim () ; if parts . starts_with ("// tidy-") { continue ; } return ; } } } ,) ; }
};
}
