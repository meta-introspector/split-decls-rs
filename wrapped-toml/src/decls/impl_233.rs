macro_rules! deps {
    () => {
        DeValue!();
        Index!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < T > Index for & T where T : Index + ? Sized , { fn index < 'r , 'i > (& self , val : & 'r DeValue < 'i >) -> Option < & 'r Spanned < DeValue < 'i > > > { (* * self) . index (val) } }
    };
}

impl_233!()