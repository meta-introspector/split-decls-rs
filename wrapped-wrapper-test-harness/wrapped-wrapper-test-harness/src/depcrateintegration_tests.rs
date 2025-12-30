// Generated macro for integration_tests (module)
macro_rules! Depcrateintegration_tests {
() => {
// Module: crate
// Provides: {"integration_tests"}
// Dependencies: {}
# [cfg (test)] mod integration_tests { use super :: * ; # [test] fn test_harness_compiles () { assert ! (true) ; } # [cfg (feature = "test-nom")] # [test] fn test_nom_wrapper_basic () { use nom :: bytes :: complete :: tag ; let result = tag :: < _ , _ , nom :: error :: Error < _ > > ("hello") ("hello world") ; assert ! (result . is_ok ()) ; } # [cfg (feature = "test-hashbrown")] # [test] fn test_hashbrown_wrapper_basic () { use hashbrown :: HashMap ; let mut map = HashMap :: new () ; map . insert ("key" , "value") ; assert_eq ! (map . get ("key") , Some (& "value")) ; } }
};
}
