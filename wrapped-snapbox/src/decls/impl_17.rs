macro_rules! deps {
    () => {
        Backtrace!();
        Result!();
        Error!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { writeln ! (f , "{}" , self . inner) ? ; if let Some (backtrace) = self . backtrace . as_ref () { writeln ! (f) ? ; writeln ! (f , "Backtrace:") ? ; writeln ! (f , "{backtrace}") ? ; } Ok (()) } }
    };
}

impl_17!()