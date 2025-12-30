// Generated macro for macro_can_load_macro_from_macro_files (function)
macro_rules! Depcrate_renderer_tests_macrosmacro_can_load_macro_from_macro_files {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"macro_can_load_macro_from_macro_files"}
// Dependencies: {}
# [test] fn macro_can_load_macro_from_macro_files () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("submacros" , "{% macro emma() %}Emma{% endmacro emma %}") , ("macros" , "{% import \"submacros\" as submacros %}{% macro hommage() %}{{ submacros::emma() }} was an amazing person!{% endmacro hommage %}") , ("parent" , "{% block main %}Someone was a terrible person!{% endblock main %} Don't you think?") , ("child" , "{% extends \"parent\" %}{% import \"macros\" as macros %}{% block main %}{{ macros::hommage() }}{% endblock main %}")]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Emma was an amazing person! Don't you think?" . to_string ()) ; }
};
}
