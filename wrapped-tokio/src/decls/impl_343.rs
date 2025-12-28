macro_rules! deps {
    () => {
        CachePadded!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < T > CachePadded < T > { # [doc = " Pads and aligns a value to the length of a cache line."] pub (crate) fn new (value : T) -> CachePadded < T > { CachePadded :: < T > { value } } }
    };
}

impl_343!()