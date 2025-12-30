// Generated macro for stateful_global_fn (function)
macro_rules! Depcrate_renderer_tests_basicstateful_global_fn {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"stateful_global_fn"}
// Dependencies: {}
# [test] fn stateful_global_fn () { fn make_tera () -> Tera { let mut tera = Tera :: default () ; tera . add_raw_template ("fn.html" , "<h1>{{ get_next() }}, {{ get_next_shared() }}, {{ get_next() }}...</h1>" ,) . unwrap () ; tera . register_function ("get_next" , Next (AtomicUsize :: new (1))) ; tera . register_function ("get_next_shared" , NEXT_GLOBAL . clone ()) ; tera } assert_eq ! (make_tera () . render ("fn.html" , & Context :: new ()) . unwrap () , "<h1>1, 1, 2...</h1>" . to_owned ()) ; assert_eq ! (make_tera () . render ("fn.html" , & Context :: new ()) . unwrap () , "<h1>1, 2, 2...</h1>" . to_owned ()) ; }
};
}
