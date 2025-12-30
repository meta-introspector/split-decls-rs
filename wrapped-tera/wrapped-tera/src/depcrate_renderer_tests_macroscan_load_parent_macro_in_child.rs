// Generated macro for can_load_parent_macro_in_child (function)
macro_rules! Depcrate_renderer_tests_macroscan_load_parent_macro_in_child {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"can_load_parent_macro_in_child"}
// Dependencies: {}
# [test] fn can_load_parent_macro_in_child () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 }}{% endmacro hello %}") , ("parent" , "{% import \"macros\" as macros %}{{ macros::hello() }}{% block bob %}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{ super() }}Hey{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "1Hey" . to_string ()) ; }
};
}
