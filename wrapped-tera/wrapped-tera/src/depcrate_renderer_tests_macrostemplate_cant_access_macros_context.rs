// Generated macro for template_cant_access_macros_context (function)
macro_rules! Depcrate_renderer_tests_macrostemplate_cant_access_macros_context {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"template_cant_access_macros_context"}
// Dependencies: {}
# [test] fn template_cant_access_macros_context () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , r#"{% import "macros" as macros %}{{ macros::empty() }}{{ quote | default(value="I'd rather have roses on my table than diamonds on my neck.") }}"#) , ("macros" , r#"{% macro empty() %}{% set_global quote = "This should not reachable from the calling template!" %}{% endmacro empty %}"#)]) . unwrap () ; let result = tera . render ("parent" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "I'd rather have roses on my table than diamonds on my neck.") ; }
};
}
