// Generated macro for render_tree_with_multiple_leaves (function)
macro_rules! Depcrate_testsrender_tree_with_multiple_leaves {
() => {
// Module: crate::tests
// Provides: {"render_tree_with_multiple_leaves"}
// Dependencies: {}
# [test] fn render_tree_with_multiple_leaves () { let tree = Tree :: new ("foo") . with_leaves (["bar" , "baz"]) ; assert_data_eq ! (format ! ("{}" , tree) , str ! [[r#"
foo
├── bar
└── baz

"#]]) ; }
};
}
