macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T > DerefMut for ThinVec < T > { fn deref_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
    };
}

impl_25!();