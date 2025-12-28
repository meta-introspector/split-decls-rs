macro_rules! deps {
    () => {
        CachedThreadLocal!();
        ThreadLocal!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T : Send + fmt :: Debug > fmt :: Debug for CachedThreadLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ThreadLocal {{ local_data: {:?} }}" , self . get ()) } }
    };
}

impl_6!();