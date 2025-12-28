macro_rules! IrPrint {
    () => {
        pub trait IrPrint < T > { fn print (t : & T , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; fn print_debug (t : & T , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result ; }
    };
}

IrPrint!();