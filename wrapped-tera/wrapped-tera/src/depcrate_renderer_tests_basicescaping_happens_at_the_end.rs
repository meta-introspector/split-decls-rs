// Generated macro for escaping_happens_at_the_end (function)
macro_rules! Depcrate_renderer_tests_basicescaping_happens_at_the_end {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"escaping_happens_at_the_end"}
// Dependencies: {}
# [test] fn escaping_happens_at_the_end () { let inputs = vec ! [# [cfg (feature = "builtins")] ("{{ url | urlencode | safe }}" , "https%3A//www.example.org/apples-%26-oranges/") , ("{{ '<html>' }}" , "&lt;html&gt;") , ("{{ '<html>' | safe }}" , "<html>") , ("{{ 'hello' | safe | replace(from='h', to='&') }}" , "&amp;ello") , ("{{ 'hello' | replace(from='h', to='&') | safe }}" , "&ello") ,] ; for (input , expected) in inputs { let mut context = Context :: new () ; context . insert ("url" , "https://www.example.org/apples-&-oranges/") ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
};
}
