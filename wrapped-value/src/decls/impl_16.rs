macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl PartialEq < Name > for str { fn eq (& self , other : & Name) -> bool { other == self } }
    };
}

impl_16!()