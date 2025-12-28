macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for AtomicCell < T > { }
    };
}

impl_274!()