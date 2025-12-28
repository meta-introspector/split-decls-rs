macro_rules! deps {
    () => {
        Slot!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Drop for Slot { fn drop (& mut self) { let ptr = self . 0 . load (Ordering :: Acquire) ; if ! ptr . is_null () { drop (unsafe { Box :: from_raw (ptr) }) ; } } }
    };
}

impl_34!();