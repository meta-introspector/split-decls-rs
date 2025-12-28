macro_rules! deps {
    () => {
        Result!();
        ValueNumber!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl ValueNumber for Value { fn to_number (& self) -> Result < f64 , () > { match * self { Value :: Number (ref i) => Ok (i . as_f64 () . unwrap ()) , _ => Err (()) , } } }
    };
}

impl_103!()