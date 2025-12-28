macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl PartialEq for Handle { fn eq (& self , other : & Handle) -> bool { if self as * const Handle == other as * const Handle { return true ; } else if self . key . is_none () || other . key . is_none () { return false ; } self . key == other . key } }
    };
}

impl_21!()