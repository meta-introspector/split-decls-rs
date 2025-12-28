macro_rules! deps {
    () => {
        LevelFilter!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl fmt :: Display for LevelFilter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { LevelFilter :: OFF => f . pad ("off") , LevelFilter :: ERROR => f . pad ("error") , LevelFilter :: WARN => f . pad ("warn") , LevelFilter :: INFO => f . pad ("info") , LevelFilter :: DEBUG => f . pad ("debug") , LevelFilter :: TRACE => f . pad ("trace") , } } }
    };
}

impl_208!();