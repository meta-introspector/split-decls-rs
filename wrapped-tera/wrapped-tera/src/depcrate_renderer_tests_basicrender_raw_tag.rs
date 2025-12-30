// Generated macro for render_raw_tag (function)
macro_rules! Depcrate_renderer_tests_basicrender_raw_tag {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"render_raw_tag"}
// Dependencies: {}
# [test] fn render_raw_tag () { let inputs = vec ! [("{% raw %}hey{% endraw %}" , "hey") , ("{% raw %}{{hey}}{% endraw %}" , "{{hey}}") , ("{% raw %}{% if true %}{% endraw %}" , "{% if true %}") ,] ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & Context :: new ()) . unwrap () , expected) ; } }
};
}
