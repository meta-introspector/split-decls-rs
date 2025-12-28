macro_rules! deps {
    () => {
        Next!();
        Function!();
        Result!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        impl Function for Next { fn call (& self , _args : & HashMap < String , Value >) -> Result < Value > { Ok (Value :: Number (self . 0 . fetch_add (1 , Ordering :: Relaxed) . into ())) } }
    };
}

impl_420!()