macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl PartialEq < str > for Key { # [inline] fn eq (& self , other : & str) -> bool { PartialEq :: eq (self . get () , other) } }
    };
}

impl_136!();