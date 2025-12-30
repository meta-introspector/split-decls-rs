// Generated macro for can_remove_whitespace_include (function)
macro_rules! Depcrate_renderer_tests_whitespacecan_remove_whitespace_include {
() => {
// Module: crate::renderer::tests::whitespace
// Provides: {"can_remove_whitespace_include"}
// Dependencies: {}
# [test] fn can_remove_whitespace_include () { let mut context = Context :: new () ; context . insert ("numbers" , & vec ! [1 , 2 , 3]) ; let inputs = vec ! [(r#"Hi {%- include "include" -%} "# , "HiIncluded") , (r#"Hi {% include "include" -%} "# , "Hi Included") , (r#"Hi {% include "include" %} "# , "Hi Included ") ,] ; for (input , expected) in inputs { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("include" , "Included") , ("tpl" , input)]) . unwrap () ; assert_eq ! (tera . render ("tpl" , & context) . unwrap () , expected) ; } }
};
}
