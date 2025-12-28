macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 's , T , I > Index < I > for SliceVec < 's , T > where I : SliceIndex < [T] > , { type Output = < I as SliceIndex < [T] > > :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { & self . deref () [index] } }
    };
}

impl_84!()