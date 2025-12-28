macro_rules! deps {
    () => {
        Entered!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl Drop for Entered < '_ > { # [inline (always)] fn drop (& mut self) { self . span . do_exit () } }
    };
}

impl_78!();