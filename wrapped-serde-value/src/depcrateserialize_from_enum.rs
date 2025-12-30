// Generated macro for serialize_from_enum (function)
macro_rules! Depcrateserialize_from_enum {
() => {
// Module: crate
// Provides: {"serialize_from_enum"}
// Dependencies: {}
# [test] fn serialize_from_enum () { # [derive (Serialize)] enum Foo { Bar , Baz (u8) , Qux { quux : u8 } , Corge (u8 , u8) , } let bar = Foo :: Bar ; assert_eq ! (to_value (& bar) . unwrap () , Value :: String ("Bar" . into ())) ; let baz = Foo :: Baz (1) ; assert_eq ! (to_value (& baz) . unwrap () , Value :: Map (vec ! [(Value :: String ("Baz" . into ()) , Value :: U8 (1))] . into_iter () . collect () ,)) ; let qux = Foo :: Qux { quux : 2 } ; assert_eq ! (to_value (& qux) . unwrap () , Value :: Map (vec ! [(Value :: String ("Qux" . into ()) , Value :: Map (vec ! [(Value :: String ("quux" . into ()) , Value :: U8 (2))] . into_iter () . collect ()))] . into_iter () . collect ())) ; let corge = Foo :: Corge (3 , 4) ; assert_eq ! (to_value (& corge) . unwrap () , Value :: Map (vec ! [(Value :: String ("Corge" . into ()) , Value :: Seq (vec ! [Value :: U8 (3) , Value :: U8 (4)]))] . into_iter () . collect ())) ; }
};
}
