// Generated macro for deserialize_into_enum (function)
macro_rules! Depcratedeserialize_into_enum {
() => {
// Module: crate
// Provides: {"deserialize_into_enum"}
// Dependencies: {}
# [test] fn deserialize_into_enum () { # [derive (Deserialize , Debug , PartialEq , Eq)] enum Foo { Bar , Baz (u8) , } let value = Value :: String ("Bar" . into ()) ; assert_eq ! (Foo :: deserialize (value) . unwrap () , Foo :: Bar) ; let value = Value :: Map (vec ! [(Value :: String ("Baz" . into ()) , Value :: U8 (1))] . into_iter () . collect () ,) ; assert_eq ! (Foo :: deserialize (value) . unwrap () , Foo :: Baz (1)) ; }
};
}
