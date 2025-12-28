macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl < T > :: core :: cmp :: PartialEq for UnionField < T > { fn eq (& self , _other : & Self) -> bool { true } }
    };
}

impl_331!()