// Generated macro for can_load_macro_in_child (function)
macro_rules! Depcrate_renderer_tests_macroscan_load_macro_in_child {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"can_load_macro_in_child"}
// Dependencies: {}
# [test] fn can_load_macro_in_child () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 }}{% endmacro hello %}") , ("parent" , "{% block bob %}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% import \"macros\" as macros %}{% block bob %}{{ macros::hello() }}{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "1" . to_string ()) ; }
};
}
