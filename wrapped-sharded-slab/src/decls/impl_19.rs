macro_rules! deps {
    () => {
        Clear!();
        Ref!();
        Config!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T , C > Drop for Ref < '_ , T , C > where T : Clear + Default , C : cfg :: Config , { fn drop (& mut self) { test_println ! ("drop Ref: try clearing data") ; let should_clear = unsafe { self . inner . release () } ; if should_clear { self . shard . clear_after_release (self . key) ; } } }
    };
}

impl_19!()