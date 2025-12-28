macro_rules! deps {
    () => {
        Page!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl Drop for Page { fn drop (& mut self) { let len = * self . allocated . get_mut () ; unsafe { (self . slot_vtable . drop_impl) (self . data . as_ptr () , len , & self . memo_types) } ; } }
    };
}

impl_335!();