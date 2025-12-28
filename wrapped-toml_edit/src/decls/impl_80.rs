macro_rules! deps {
    () => {
        Index!();
        Item!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < I > ops :: Index < I > for Item where I : Index , { type Output = Self ; fn index (& self , index : I) -> & Self { index . index (self) . expect ("index not found") } }
    };
}

impl_80!()