macro_rules! deps {
    () => {
        Value!();
        Index!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < I > ops :: IndexMut < I > for Value where I : Index , { fn index_mut (& mut self , index : I) -> & mut Self { self . get_mut (index) . expect ("index not found") } }
    };
}

impl_56!();