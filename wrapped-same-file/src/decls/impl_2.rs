macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Drop for Handle { fn drop (& mut self) { if self . is_std { let _ = self . file . take () . unwrap () . into_raw_fd () ; } } }
    };
}

impl_2!()