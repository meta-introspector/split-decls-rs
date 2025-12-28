macro_rules! deps {
    () => {
        DeValue!();
        Index!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < I > ops :: Index < I > for DeValue < '_ > where I : Index , { type Output = Spanned < Self > ; fn index (& self , index : I) -> & Spanned < Self > { self . get (index) . expect ("index not found") } }
    };
}

impl_223!()