macro_rules! deps {
    () => {
        KeyMut!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl PartialEq < str > for KeyMut < '_ > { # [inline] fn eq (& self , other : & str) -> bool { PartialEq :: eq (self . get () , other) } }
    };
}

impl_148!();