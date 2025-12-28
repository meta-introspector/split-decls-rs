macro_rules! deps {
    () => {
        Index!();
        DeArray!();
        DeValue!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'i , I : core :: slice :: SliceIndex < [Spanned < DeValue < 'i > >] > > core :: ops :: Index < I > for DeArray < 'i > { type Output = I :: Output ; # [inline] fn index (& self , index : I) -> & Self :: Output { self . items . index (index) } }
    };
}

impl_189!()