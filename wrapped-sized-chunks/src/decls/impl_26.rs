macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < A , T > Ord for InlineArray < A , T > where A : Ord , { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
    };
}

impl_26!()