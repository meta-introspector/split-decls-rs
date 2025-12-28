macro_rules! deps {
    () => {
        RecursiveGuard!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Drop for RecursiveGuard { fn drop (& mut self) { self . 0 . with (| is_empty | is_empty . store (true , Ordering :: Relaxed)) ; } }
    };
}

impl_11!()