macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < A : Array , I : SliceIndex < [A :: Item] > > Index < I > for TinyVec < A > { type Output = < I as SliceIndex < [A :: Item] > > :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { & self . deref () [index] } }
    };
}

impl_124!()