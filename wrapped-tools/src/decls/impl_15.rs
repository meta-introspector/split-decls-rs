macro_rules! deps {
    () => {
        AutoRevertToPreviousCWD!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Drop for AutoRevertToPreviousCWD { fn drop (& mut self) { env :: set_current_dir (& self . 0) . unwrap () ; } }
    };
}

impl_15!()