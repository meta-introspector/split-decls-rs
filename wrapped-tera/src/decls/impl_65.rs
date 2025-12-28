macro_rules! deps {
    () => {
        Result!();
        Filter!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < F > Filter for F where F : Fn (& Value , & HashMap < String , Value >) -> Result < Value > + Sync + Send , { fn filter (& self , value : & Value , args : & HashMap < String , Value >) -> Result < Value > { self (value , args) } }
    };
}

impl_65!()