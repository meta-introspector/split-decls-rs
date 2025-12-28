macro_rules! deps {
    () => {
        UnordCollection!();
        UnordItems!();
        ExtendUnord!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        impl < C : Extend < T > + UnordCollection , T > ExtendUnord < T > for C { # [inline] fn extend_unord < I : Iterator < Item = T > > (& mut self , items : UnordItems < T , I >) { self . extend (items . 0) } }
    };
}

impl_649!();