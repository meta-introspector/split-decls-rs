// Generated macro for unit_tests (module)
macro_rules! Depcrateunit_tests {
() => {
// Module: crate
// Provides: {"unit_tests"}
// Dependencies: {}
# [cfg (test)] mod unit_tests { use super :: * ; # [test] fn test_no_panic_on_format_snippet_and_format_code_block () { let snippet = "let" ; assert ! (format_snippet (snippet , & Config :: default () , false) . is_none ()) ; assert ! (format_code_block (snippet , & Config :: default () , false) . is_none ()) ; } fn test_format_inner < F > (formatter : F , input : & str , expected : & str) -> bool where F : Fn (& str , & Config , bool) -> Option < FormattedSnippet > , { let output = formatter (input , & Config :: default () , false) ; output . is_some () && output . unwrap () . snippet == expected } # [test] fn test_format_snippet () { let snippet = "fn main() { println!(\"hello, world\"); }" ; # [cfg (not (windows))] let expected = "fn main() {\n    \
                        println!(\"hello, world\");\n\
                        }\n" ; # [cfg (windows)] let expected = "fn main() {\r\n    \
                        println!(\"hello, world\");\r\n\
                        }\r\n" ; assert ! (test_format_inner (format_snippet , snippet , expected)) ; } # [test] fn test_format_code_block_fail () { # [rustfmt :: skip] let code_block = "this_line_is_100_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx(x, y, z);" ; assert ! (format_code_block (code_block , & Config :: default () , false) . is_none ()) ; } # [test] fn test_format_code_block () { let code_block = "let x=3;" ; let expected = "let x = 3;" ; assert ! (test_format_inner (format_code_block , code_block , expected)) ; let code_block = "let (nested_shape, extend) = if !parent_rewrite_contains_newline && is_continuable(&parent) {
(
chain_indent(context, shape.add_offset(parent_rewrite.len())),
context.config.indent_style() == IndentStyle::Visual || is_small_parent,
)
} else if is_block_expr(context, &parent, &parent_rewrite) {
match context.config.indent_style() {
// Try to put the first child on the same line with parent's last line
IndentStyle::Block => (parent_shape.block_indent(context.config.tab_spaces()), true),
// The parent is a block, so align the rest of the chain with the closing
// brace.
IndentStyle::Visual => (parent_shape, false),
}
} else {
(
chain_indent(context, shape.add_offset(parent_rewrite.len())),
false,
)
};
" ; let expected = "let (nested_shape, extend) = if !parent_rewrite_contains_newline && is_continuable(&parent) {
    (
        chain_indent(context, shape.add_offset(parent_rewrite.len())),
        context.config.indent_style() == IndentStyle::Visual || is_small_parent,
    )
} else if is_block_expr(context, &parent, &parent_rewrite) {
    match context.config.indent_style() {
        // Try to put the first child on the same line with parent's last line
        IndentStyle::Block => (parent_shape.block_indent(context.config.tab_spaces()), true),
        // The parent is a block, so align the rest of the chain with the closing
        // brace.
        IndentStyle::Visual => (parent_shape, false),
    }
} else {
    (
        chain_indent(context, shape.add_offset(parent_rewrite.len())),
        false,
    )
};" ; assert ! (test_format_inner (format_code_block , code_block , expected)) ; } }
};
}
