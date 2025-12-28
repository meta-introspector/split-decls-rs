macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > Drop for Entry < '_ , T , C > { fn drop (& mut self) { let should_remove = unsafe { self . inner . release () } ; if should_remove { self . shard . clear_after_release (self . key) } } }
    };
}

impl_22!()