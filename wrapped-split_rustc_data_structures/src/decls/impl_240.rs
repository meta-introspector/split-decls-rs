macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl < 'a , T > PartialEq for Interned < 'a , T > { # [inline] fn eq (& self , other : & Self) -> bool { ptr :: eq (self . 0 , other . 0) } }
    };
}

impl_240!()