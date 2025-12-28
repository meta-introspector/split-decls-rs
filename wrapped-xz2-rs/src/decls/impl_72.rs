macro_rules! deps {
    () => {
        XzDecoder!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < W : Write > Drop for XzDecoder < W > { fn drop (& mut self) { if self . obj . is_some () { let _ = self . try_finish () ; } } }
    };
}

impl_72!();