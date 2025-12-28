macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl PartialEq for BSTR { fn eq (& self , other : & Self) -> bool { self . deref () == other . deref () } }
    };
}

impl_12!()