macro_rules! deps {
    () => {
        KeyMut!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl PartialEq < String > for KeyMut < '_ > { # [inline] fn eq (& self , other : & String) -> bool { PartialEq :: eq (self . get () , other . as_str ()) } }
    };
}

impl_150!()