macro_rules! deps {
    () => {
        WakeFd!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Drop for WakeFd { fn drop (& mut self) { unsafe { libc :: close (self . fd) ; } } }
    };
}

impl_72!()