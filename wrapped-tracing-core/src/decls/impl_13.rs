macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : ? Sized + fmt :: Debug > fmt :: Debug for Mutex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_lock () { Some (guard) => write ! (f , "Mutex {{ data: ") . and_then (| () | (& * guard) . fmt (f)) . and_then (| () | write ! (f , "}}")) , None => write ! (f , "Mutex {{ <locked> }}") , } } }
    };
}

impl_13!()