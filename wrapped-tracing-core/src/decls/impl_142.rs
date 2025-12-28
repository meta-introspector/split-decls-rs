macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > crate :: sealed :: Sealed for & 'a mut T where T : Value + crate :: sealed :: Sealed + 'a { }
    };
}

impl_142!();