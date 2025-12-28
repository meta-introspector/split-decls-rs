macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! deserialize_into_enum {
    () => {
        deps!();
        # [test] fn deserialize_into_enum () { # [derive (Deserialize , Debug , PartialEq , Eq)] enum Foo { Bar , Baz (u8) , } let value = Value :: String ("Bar" . into ()) ; assert_eq ! (Foo :: deserialize (value) . unwrap () , Foo :: Bar) ; let value = Value :: Map (vec ! [(Value :: String ("Baz" . into ()) , Value :: U8 (1))] . into_iter () . collect () ,) ; assert_eq ! (Foo :: deserialize (value) . unwrap () , Foo :: Baz (1)) ; }
    };
}

deserialize_into_enum!();