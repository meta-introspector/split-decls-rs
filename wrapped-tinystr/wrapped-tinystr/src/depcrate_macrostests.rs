// Generated macro for tests (module)
macro_rules! Depcrate_macrostests {
() => {
// Module: crate::macros
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_macro_construction () { let s1 = tinystr ! (8 , "foobar") ; assert_eq ! (&* s1 , "foobar") ; let s1 = tinystr ! (12 , "foobarbaz") ; assert_eq ! (&* s1 , "foobarbaz") ; } }
};
}
