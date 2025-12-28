macro_rules! deps {
    () => {
        XzEncoder!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < W : Write > Drop for XzEncoder < W > { fn drop (& mut self) { if self . obj . is_some () { let _ = self . try_finish () ; } } }
    };
}

impl_66!();