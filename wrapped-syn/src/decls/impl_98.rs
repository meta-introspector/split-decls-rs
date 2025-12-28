macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'a > PartialEq for Cursor < 'a > { fn eq (& self , other : & Self) -> bool { ptr :: eq (self . ptr , other . ptr) } }
    };
}

impl_98!()