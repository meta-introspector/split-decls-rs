// Generated macro for render_tree_with_multiline_leaf (function)
macro_rules! Depcrate_testsrender_tree_with_multiline_leaf {
() => {
// Module: crate::tests
// Provides: {"render_tree_with_multiline_leaf"}
// Dependencies: {}
# [test] fn render_tree_with_multiline_leaf () { let tree = Tree :: new ("foo") . with_leaves ([Tree :: new ("hello\nworld") . with_multiline (true) , Tree :: new ("goodbye\nworld") . with_multiline (true) ,]) ; assert_data_eq ! (format ! ("{}" , tree) , str ! [[r#"
foo
├── hello
│   world
└── goodbye
    world

"#]]) ; }
};
}
