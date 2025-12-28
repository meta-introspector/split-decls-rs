macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < A : Array , I : SliceIndex < [A :: Item] > > Index < I > for ArrayVec < A > { type Output = < I as SliceIndex < [A :: Item] > > :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { & self . deref () [index] } }
    };
}

impl_12!()