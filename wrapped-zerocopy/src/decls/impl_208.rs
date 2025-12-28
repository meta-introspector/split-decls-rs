macro_rules! deps {
    () => {
        TryFromBytes!();
        ValidityError!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < Src : PartialEq , Dst : ? Sized + TryFromBytes > PartialEq for ValidityError < Src , Dst > { # [inline] fn eq (& self , other : & Self) -> bool { self . src == other . src } }
    };
}

impl_208!()