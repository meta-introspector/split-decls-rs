macro_rules! deps {
    () => {
        MaybeTempDir!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        impl Drop for MaybeTempDir { fn drop (& mut self) { let dir = unsafe { ManuallyDrop :: take (& mut self . dir) } ; if self . keep { let _ = dir . keep () ; } } }
    };
}

impl_595!();