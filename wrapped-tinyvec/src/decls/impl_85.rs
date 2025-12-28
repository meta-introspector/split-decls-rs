macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < 's , T , I > IndexMut < I > for SliceVec < 's , T > where I : SliceIndex < [T] > , { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { & mut self . deref_mut () [index] } }
    };
}

impl_85!()