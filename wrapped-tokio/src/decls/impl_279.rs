macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < T > Drop for AtomicCell < T > { fn drop (& mut self) { let _ = self . take () ; } }
    };
}

impl_279!();