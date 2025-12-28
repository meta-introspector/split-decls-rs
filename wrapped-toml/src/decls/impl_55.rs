macro_rules! deps {
    () => {
        Value!();
        Index!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < I > ops :: Index < I > for Value where I : Index , { type Output = Self ; fn index (& self , index : I) -> & Self { self . get (index) . expect ("index not found") } }
    };
}

impl_55!();