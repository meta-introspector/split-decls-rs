macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < A : Array , I : SliceIndex < [A :: Item] > > IndexMut < I > for TinyVec < A > { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { & mut self . deref_mut () [index] } }
    };
}

impl_125!()