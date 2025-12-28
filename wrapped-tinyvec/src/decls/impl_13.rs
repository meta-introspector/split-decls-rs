macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < A : Array , I : SliceIndex < [A :: Item] > > IndexMut < I > for ArrayVec < A > { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { & mut self . deref_mut () [index] } }
    };
}

impl_13!()