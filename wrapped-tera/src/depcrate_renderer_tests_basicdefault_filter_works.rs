// Generated macro for default_filter_works (function)
macro_rules! Depcrate_renderer_tests_basicdefault_filter_works {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"default_filter_works"}
// Dependencies: {}
# [test] fn default_filter_works () { let mut context = Context :: new () ; let i : Option < usize > = None ; context . insert ("existing" , "hello") ; context . insert ("null" , & i) ; let inputs = vec ! [(r#"{{ existing | default(value="hey") }}"# , "hello") , (r#"{{ val | default(value=1) }}"# , "1") , (r#"{{ val | default(value="hey") | capitalize }}"# , "Hey") , (r#"{{ obj.val | default(value="hey") | capitalize }}"# , "Hey") , (r#"{{ obj.val | default(value="hey") | capitalize }}"# , "Hey") , (r#"{{ not admin | default(value=false) }}"# , "true") , (r#"{{ not admin | default(value=true) }}"# , "false") , (r#"{{ null | default(value=true) }}"# , "true") , (r#"{{ null | default(value="hey") | capitalize }}"# , "Hey") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
};
}
