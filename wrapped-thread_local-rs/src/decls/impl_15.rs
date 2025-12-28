macro_rules! deps {
    () => {
        ThreadLocal!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : Send + fmt :: Debug > fmt :: Debug for ThreadLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ThreadLocal {{ local_data: {:?} }}" , self . get ()) } }
    };
}

impl_15!()