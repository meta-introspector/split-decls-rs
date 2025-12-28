macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Drop for Pool { fn drop (& mut self) { self . join () ; unsafe { CloseThreadpoolCleanupGroup (self . 0 . CleanupGroup) ; CloseThreadpool (self . 0 . Pool) ; } } }
    };
}

impl_29!()