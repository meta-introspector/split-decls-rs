macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > crate :: sealed :: Sealed for & 'a T where T : Value + crate :: sealed :: Sealed + 'a { }
    };
}

impl_140!();