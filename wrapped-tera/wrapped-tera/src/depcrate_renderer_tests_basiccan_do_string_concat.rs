// Generated macro for can_do_string_concat (function)
macro_rules! Depcrate_renderer_tests_basiccan_do_string_concat {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"can_do_string_concat"}
// Dependencies: {}
# [test] fn can_do_string_concat () { let mut context = Context :: new () ; context . insert ("a_string" , "hello") ; context . insert ("another_string" , "xXx") ; context . insert ("an_int" , & 1) ; context . insert ("a_float" , & 3.18) ; let inputs = vec ! [(r#"{{ "hello" ~ " world" }}"# , "hello world") , (r#"{{ "hello" ~ 1 }}"# , "hello1") , (r#"{{ "hello" ~ 3.18 }}"# , "hello3.18") , (r#"{{ 3.18 ~ "hello"}}"# , "3.18hello") , (r#"{{ "hello" ~ get_string() }}"# , "helloHello") , (r#"{{ get_string() ~ "hello" }}"# , "Hellohello") , (r#"{{ get_string() ~ 3.18 }}"# , "Hello3.18") , (r#"{{ a_string ~ " world" }}"# , "hello world") , (r#"{{ a_string ~ ' world ' ~ another_string }}"# , "hello world xXx") , (r#"{{ a_string ~ another_string }}"# , "helloxXx") , (r#"{{ a_string ~ an_int }}"# , "hello1") , (r#"{{ a_string ~ a_float }}"# , "hello3.18") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
};
}
