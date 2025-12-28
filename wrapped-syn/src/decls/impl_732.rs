macro_rules! deps {
    () => {
        Result!();
        ThreadBound!();
    };
}

macro_rules! impl_732 {
    () => {
        deps!();
        impl < T : Debug > Debug for ThreadBound < T > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self . get () { Some (value) => Debug :: fmt (value , formatter) , None => formatter . write_str ("unknown") , } } }
    };
}

impl_732!();