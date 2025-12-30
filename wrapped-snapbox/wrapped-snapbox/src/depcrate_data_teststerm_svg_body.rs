// Generated macro for term_svg_body (module)
macro_rules! Depcrate_data_teststerm_svg_body {
() => {
// Module: crate::data::tests
// Provides: {"term_svg_body"}
// Dependencies: {}
# [cfg (feature = "term-svg")] mod term_svg_body { use super :: super :: * ; # [test] fn empty () { let input = "" ; let expected = None ; let actual = term_svg_body (input) ; assert_eq ! (expected , actual) ; } # [test] fn no_open_tag () { let input = "hello
</text>
world!" ; let expected = None ; let actual = term_svg_body (input) ; assert_eq ! (expected , actual) ; } # [test] fn unclosed_open_text () { let input = "
Hello
<text
world!" ; let expected = None ; let actual = term_svg_body (input) ; assert_eq ! (expected , actual) ; } # [test] fn capture_one () { let input = "
Hello
<text>
world
</text>
Oh" ; let expected = Some ("<text>
world
</text>
" ,) ; let actual = term_svg_body (input) ; assert_eq ! (expected , actual) ; } # [test] fn no_end_tag () { let input = "
Hello
<text>
world" ; let expected = Some ("<text>
world" ,) ; let actual = term_svg_body (input) ; assert_eq ! (expected , actual) ; } }
};
}
