macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl PartialEq < Name > for & '_ str { fn eq (& self , other : & Name) -> bool { other == self } }
    };
}

impl_95!()