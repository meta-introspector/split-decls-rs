macro_rules! deps {
    () => {
        MockLayer!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl fmt :: Debug for MockLayer { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("ExpectSubscriber") ; s . field ("name" , & self . name) ; if let Ok (expected) = self . expected . try_lock () { s . field ("expected" , & expected) ; } else { s . field ("expected" , & format_args ! ("<locked>")) ; } if let Ok (current) = self . current . try_lock () { s . field ("current" , & format_args ! ("{:?}" , & current)) ; } else { s . field ("current" , & format_args ! ("<locked>")) ; } s . finish () } }
    };
}

impl_88!();