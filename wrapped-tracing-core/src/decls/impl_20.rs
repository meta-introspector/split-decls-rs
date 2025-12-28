macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for Once < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . r#try () { Some (s) => write ! (f , "Once {{ data: ") . and_then (| () | s . fmt (f)) . and_then (| () | write ! (f , "}}")) , None => write ! (f , "Once {{ <uninitialized> }}") , } } }
    };
}

impl_20!()