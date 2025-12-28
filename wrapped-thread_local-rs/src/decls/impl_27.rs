macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T > Drop for Entry < T > { fn drop (& mut self) { if * self . present . get_mut () { unsafe { MaybeUninit :: assume_init_drop (& mut * self . value . get ()) ; } } } }
    };
}

impl_27!();