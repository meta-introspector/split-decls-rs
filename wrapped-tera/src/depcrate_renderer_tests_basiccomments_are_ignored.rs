// Generated macro for comments_are_ignored (function)
macro_rules! Depcrate_renderer_tests_basiccomments_are_ignored {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"comments_are_ignored"}
// Dependencies: {}
# [test] fn comments_are_ignored () { let inputs = vec ! [("Hello {# comment #}world" , "Hello world") , ("Hello {# comment {# nested #}world" , "Hello world") , ("My name {# was {{ name }} #}is No One." , "My name is No One.") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & Context :: new ()) . unwrap () , expected) ; } }
};
}
