macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < A , T > PartialOrd for InlineArray < A , T > where A : PartialOrd , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
    };
}

impl_25!()