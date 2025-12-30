// Generated macro for can_set_variables_in_included_templates (function)
macro_rules! Depcrate_renderer_tests_basiccan_set_variables_in_included_templates {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"can_set_variables_in_included_templates"}
// Dependencies: {}
# [test] fn can_set_variables_in_included_templates () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("world" , r#"{% set a = "world" %}{{a}}"#) , ("hello" , "<h1>Hello {% include \"world\" %}</h1>") ,]) . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello world</h1>" . to_owned ()) ; }
};
}
