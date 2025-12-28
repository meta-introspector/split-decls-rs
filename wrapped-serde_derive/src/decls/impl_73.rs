macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Display for Name { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . value , formatter) } }
    };
}

impl_73!()