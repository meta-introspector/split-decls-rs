macro_rules! deps {
    () => {
        OrderedF64!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Ord for OrderedF64 { fn cmp (& self , other : & OrderedF64) -> Ordering { self . partial_cmp (other) . unwrap () } }
    };
}

impl_129!()