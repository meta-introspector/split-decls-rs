macro_rules! deps {
    () => {
        MutexGuardWrapper!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Drop for MutexGuardWrapper < '_ > { fn drop (& mut self) { # [cfg (feature = "logging")] debug ! ("End serial") ; self . locks . arc . condvar . notify_one () ; } }
    };
}

impl_20!();