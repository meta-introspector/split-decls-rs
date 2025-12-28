macro_rules! deps {
    () => {
        WeakDispatch!();
        Kind!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl fmt :: Debug for WeakDispatch { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . subscriber { Kind :: Scoped (ref s) => f . debug_tuple ("WeakDispatch::Scoped") . field (& format_args ! ("{:p}" , s)) . finish () , Kind :: Global (s) => f . debug_tuple ("WeakDispatch::Global") . field (& format_args ! ("{:p}" , s)) . finish () , } } }
    };
}

impl_91!();