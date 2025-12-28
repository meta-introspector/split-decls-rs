macro_rules! deps {
    () => {
        ColorLevel!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for ColorLevel < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self . 0 { Level :: TRACE => Color :: Purple . bold () . paint ("TRACE") , Level :: DEBUG => Color :: Blue . bold () . paint ("DEBUG") , Level :: INFO => Color :: Green . bold () . paint (" INFO") , Level :: WARN => Color :: Rgb (252 , 234 , 160) . bold () . paint (" WARN") , Level :: ERROR => Color :: Red . bold () . paint ("ERROR") , } . fmt (f) } }
    };
}

impl_16!()