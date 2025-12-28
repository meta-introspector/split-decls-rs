macro_rules! deps {
    () => {
        Config!();
        Addr!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < C : cfg :: Config > fmt :: Debug for Addr < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Addr") . field ("addr" , & format_args ! ("{:#0x}" , & self . addr)) . field ("index" , & self . index ()) . field ("offset" , & self . offset ()) . finish () } }
    };
}

impl_137!()