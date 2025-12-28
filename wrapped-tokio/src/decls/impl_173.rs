macro_rules! deps {
    () => {
        AtomicU16!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        unsafe impl Send for AtomicU16 { }
    };
}

impl_173!()