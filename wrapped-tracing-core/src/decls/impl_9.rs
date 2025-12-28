macro_rules! deps {
    () => {
        Mutex!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Sync for Mutex < T > { }
    };
}

impl_9!()