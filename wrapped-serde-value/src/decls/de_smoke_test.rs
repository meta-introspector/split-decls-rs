macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! de_smoke_test {
    () => {
        deps!();
        # [test] fn de_smoke_test () { let value = Value :: Option (Some (Box :: new (Value :: Seq (vec ! [Value :: U16 (8) , Value :: Char ('a') , Value :: F32 (1.0) , Value :: String ("hello" . into ()) , Value :: Map (vec ! [(Value :: Bool (false) , Value :: Unit) , (Value :: Bool (true) , Value :: Newtype (Box :: new (Value :: Bytes (b"hi" . as_ref () . into ()))) ,) ,] . into_iter () . collect () ,) ,])))) ; let value_de = Value :: deserialize (value . clone ()) . unwrap () ; assert_eq ! (value_de , value) ; }
    };
}

de_smoke_test!()