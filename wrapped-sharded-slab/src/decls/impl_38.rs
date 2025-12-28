macro_rules! deps {
    () => {
        OwnedRefMut!();
        Clear!();
        Config!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T , C > Drop for OwnedRefMut < T , C > where T : Clear + Default , C : cfg :: Config , { fn drop (& mut self) { test_println ! ("drop OwnedRefMut: try clearing data") ; let should_clear = unsafe { self . inner . release () } ; if should_clear { if let Some (shard) = self . shard () { shard . clear_after_release (self . key) ; } else { test_println ! ("-> shard does not exist! THIS IS A BUG") ; debug_assert ! (std :: thread :: panicking () , "[internal error] tried to drop an `OwnedRefMut` to a slot on a shard that never existed!") ; } } } }
    };
}

impl_38!();