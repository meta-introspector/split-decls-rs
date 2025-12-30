// Generated macro for deserialize_newtype (function)
macro_rules! Depcratedeserialize_newtype {
() => {
// Module: crate
// Provides: {"deserialize_newtype"}
// Dependencies: {}
# [test] fn deserialize_newtype () { # [derive (Debug , Deserialize , PartialEq)] struct Foo (i32) ; let input = Value :: I32 (5) ; let foo = Foo :: deserialize (input) . unwrap () ; assert_eq ! (foo , Foo (5)) ; }
};
}
