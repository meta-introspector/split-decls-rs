macro_rules! deps {
    () => {
        NewSpan!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl fmt :: Debug for NewSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("NewSpan") ; if let Some (name) = self . span . name () { s . field ("name" , & name) ; } if let Some (level) = self . span . level () { s . field ("level" , & format_args ! ("{:?}" , level)) ; } if let Some (target) = self . span . target () { s . field ("target" , & target) ; } if let Some (ref parent) = self . ancestry { s . field ("parent" , & format_args ! ("{:?}" , parent)) ; } if ! self . fields . is_empty () { s . field ("fields" , & self . fields) ; } s . finish () } }
    };
}

impl_59!();