macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl Display for Name { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { Display :: fmt (& self . 0 , f) } }
    };
}

impl_89!()