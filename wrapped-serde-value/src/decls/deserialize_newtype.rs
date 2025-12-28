macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! deserialize_newtype {
    () => {
        deps!();
        # [test] fn deserialize_newtype () { # [derive (Debug , Deserialize , PartialEq)] struct Foo (i32) ; let input = Value :: I32 (5) ; let foo = Foo :: deserialize (input) . unwrap () ; assert_eq ! (foo , Foo (5)) ; }
    };
}

deserialize_newtype!();