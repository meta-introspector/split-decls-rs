// Generated macro for render_tree_root (function)
macro_rules! Depcrate_testsrender_tree_root {
() => {
// Module: crate::tests
// Provides: {"render_tree_root"}
// Dependencies: {}
# [test] fn render_tree_root () { let tree = Tree :: new ("foo") ; assert_data_eq ! (format ! ("{}" , tree) , str ! [[r#"
foo

"#]]) ; }
};
}
