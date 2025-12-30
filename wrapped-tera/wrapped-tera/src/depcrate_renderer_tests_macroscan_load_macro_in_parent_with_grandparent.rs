// Generated macro for can_load_macro_in_parent_with_grandparent (function)
macro_rules! Depcrate_renderer_tests_macroscan_load_macro_in_parent_with_grandparent {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"can_load_macro_in_parent_with_grandparent"}
// Dependencies: {}
# [test] fn can_load_macro_in_parent_with_grandparent () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 }}{% endmacro hello %}") , ("grandparent" , "{% block bob %}{% endblock bob %}") , ("parent" , "{% extends \"grandparent\" %}{% import \"macros\" as macros %}{% block bob %}{{ macros::hello() }} - Hey{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{ super() }}{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "1 - Hey" . to_string ()) ; }
};
}
