macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl PartialEq for HSTRING { fn eq (& self , other : & Self) -> bool { self . deref () == other . deref () } }
    };
}

impl_39!()