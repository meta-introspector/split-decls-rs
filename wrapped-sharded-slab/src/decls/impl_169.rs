macro_rules! deps {
    () => {
        Tid!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < C > fmt :: Debug for Tid < C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_poisoned () { f . debug_tuple ("Tid") . field (& format_args ! ("<poisoned>")) . finish () } else { f . debug_tuple ("Tid") . field (& format_args ! ("{}" , self . id)) . finish () } } }
    };
}

impl_169!()