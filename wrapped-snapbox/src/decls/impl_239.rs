macro_rules! deps {
    () => {
        RedactedValueInner!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl PartialEq for RedactedValueInner { fn eq (& self , other : & Self) -> bool { self . as_cmp () . eq (& other . as_cmp ()) } }
    };
}

impl_239!()