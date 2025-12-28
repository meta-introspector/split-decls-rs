macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Debug for Version { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = formatter . debug_struct ("Version") ; debug . field ("major" , & self . major) . field ("minor" , & self . minor) . field ("patch" , & self . patch) ; if ! self . pre . is_empty () { debug . field ("pre" , & self . pre) ; } if ! self . build . is_empty () { debug . field ("build" , & self . build) ; } debug . finish () } }
    };
}

impl_6!()