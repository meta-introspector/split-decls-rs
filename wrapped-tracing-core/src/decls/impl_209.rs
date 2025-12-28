macro_rules! deps {
    () => {
        LevelFilter!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl fmt :: Debug for LevelFilter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { LevelFilter :: OFF => f . pad ("LevelFilter::OFF") , LevelFilter :: ERROR => f . pad ("LevelFilter::ERROR") , LevelFilter :: WARN => f . pad ("LevelFilter::WARN") , LevelFilter :: INFO => f . pad ("LevelFilter::INFO") , LevelFilter :: DEBUG => f . pad ("LevelFilter::DEBUG") , LevelFilter :: TRACE => f . pad ("LevelFilter::TRACE") , } } }
    };
}

impl_209!();