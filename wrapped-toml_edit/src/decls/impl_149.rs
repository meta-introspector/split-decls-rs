macro_rules! deps {
    () => {
        KeyMut!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 's > PartialEq < & 's str > for KeyMut < 's > { # [inline] fn eq (& self , other : & & str) -> bool { PartialEq :: eq (self . get () , * other) } }
    };
}

impl_149!();