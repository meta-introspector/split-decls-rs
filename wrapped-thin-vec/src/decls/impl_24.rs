macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < T > Deref for ThinVec < T > { type Target = [T] ; fn deref (& self) -> & [T] { self . as_slice () } }
    };
}

impl_24!()