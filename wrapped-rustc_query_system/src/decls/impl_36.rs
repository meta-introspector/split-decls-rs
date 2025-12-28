macro_rules! deps {
    () => {
        EdgesVec!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Deref for EdgesVec { type Target = [DepNodeIndex] ; # [inline] fn deref (& self) -> & Self :: Target { self . edges . as_slice () } }
    };
}

impl_36!()