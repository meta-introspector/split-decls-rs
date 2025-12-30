// Generated macro for render_tree_with_leaves (function)
macro_rules! Depcrate_testsrender_tree_with_leaves {
() => {
// Module: crate::tests
// Provides: {"render_tree_with_leaves"}
// Dependencies: {}
# [test] fn render_tree_with_leaves () { let tree = Tree :: new ("foo") . with_leaves ([Tree :: new ("bar") . with_leaves (["baz"])]) ; assert_data_eq ! (format ! ("{}" , tree) , str ! [[r#"
foo
└── bar
    └── baz

"#]]) ; }
};
}
