macro_rules! deps {
    () => {
        Result!();
        Function!();
        SharedNext!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl Function for SharedNext { fn call (& self , args : & HashMap < String , Value >) -> Result < Value > { self . 0 . call (args) } }
    };
}

impl_422!()