macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl fmt :: Display for Level { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Level :: TRACE => f . pad ("TRACE") , Level :: DEBUG => f . pad ("DEBUG") , Level :: INFO => f . pad ("INFO") , Level :: WARN => f . pad ("WARN") , Level :: ERROR => f . pad ("ERROR") , } } }
    };
}

impl_200!()