macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : PartialEq > PartialEq for Spanned < T > { fn eq (& self , other : & Self) -> bool { self . value . eq (& other . value) } }
    };
}

impl_14!()