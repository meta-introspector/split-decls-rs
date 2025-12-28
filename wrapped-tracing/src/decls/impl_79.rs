macro_rules! deps {
    () => {
        EnteredSpan!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Drop for EnteredSpan { # [inline (always)] fn drop (& mut self) { self . span . do_exit () } }
    };
}

impl_79!()