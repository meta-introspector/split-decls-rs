// Generated macro for strips_trailing_slash (function)
macro_rules! Depcrate_dir_testsstrips_trailing_slash {
() => {
// Module: crate::dir::tests
// Provides: {"strips_trailing_slash"}
// Dependencies: {}
# [test] fn strips_trailing_slash () { let path = std :: path :: Path :: new ("/foo/bar/") ; let rendered = path . display () . to_string () ; assert_eq ! (rendered . as_bytes () [rendered . len () - 1] , b'/') ; let stripped = strip_trailing_slash (path) ; let rendered = stripped . display () . to_string () ; assert_eq ! (rendered . as_bytes () [rendered . len () - 1] , b'r') ; }
};
}
