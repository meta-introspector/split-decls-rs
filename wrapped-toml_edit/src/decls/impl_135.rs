macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl PartialEq for Key { # [inline] fn eq (& self , other : & Self) -> bool { PartialEq :: eq (self . get () , other . get ()) } }
    };
}

impl_135!();