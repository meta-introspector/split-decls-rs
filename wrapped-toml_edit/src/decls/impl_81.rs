macro_rules! deps {
    () => {
        Item!();
        Index!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < I > ops :: IndexMut < I > for Item where I : Index , { fn index_mut (& mut self , index : I) -> & mut Self { index . index_mut (self) . expect ("index not found") } }
    };
}

impl_81!()