macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl PartialEq < String > for Key { # [inline] fn eq (& self , other : & String) -> bool { PartialEq :: eq (self . get () , other . as_str ()) } }
    };
}

impl_138!()