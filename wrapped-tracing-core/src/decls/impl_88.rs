macro_rules! deps {
    () => {
        Dispatch!();
        Kind!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl fmt :: Debug for Dispatch { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . subscriber { Kind :: Scoped (ref s) => f . debug_tuple ("Dispatch::Scoped") . field (& format_args ! ("{:p}" , s)) . finish () , Kind :: Global (s) => f . debug_tuple ("Dispatch::Global") . field (& format_args ! ("{:p}" , s)) . finish () , } } }
    };
}

impl_88!();