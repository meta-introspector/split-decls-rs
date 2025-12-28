macro_rules! deps {
    () => {
        Cancelled!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl std :: fmt :: Display for Cancelled { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let why = match self { Cancelled :: PendingWrite => "pending write" , Cancelled :: PropagatedPanic => "propagated panic" , } ; f . write_str ("cancelled because of ") ? ; f . write_str (why) } }
    };
}

impl_37!()