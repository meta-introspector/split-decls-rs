macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Ord for SmolStr { fn cmp (& self , other : & SmolStr) -> Ordering { self . as_str () . cmp (other . as_str ()) } }
    };
}

impl_16!()