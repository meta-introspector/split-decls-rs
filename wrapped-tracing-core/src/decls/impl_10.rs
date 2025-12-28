macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for Mutex < T > { }
    };
}

impl_10!();