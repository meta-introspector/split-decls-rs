macro_rules! deps {
    () => {
        Test!();
        Result!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < F > Test for F where F : Fn (Option < & Value > , & [Value]) -> Result < bool > + Sync + Send , { fn test (& self , value : Option < & Value > , args : & [Value]) -> Result < bool > { self (value , args) } }
    };
}

impl_77!();