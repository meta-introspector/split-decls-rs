macro_rules! deps {
    () => {
        Result!();
        Function!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < F > Function for F where F : Fn (& HashMap < String , Value >) -> Result < Value > + Sync + Send , { fn call (& self , args : & HashMap < String , Value >) -> Result < Value > { self (args) } }
    };
}

impl_68!();