macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < A , const N : usize > Drop for RingBuffer < A , N > { # [inline] fn drop (& mut self) { if core :: mem :: needs_drop :: < A > () { for i in self . range () { unsafe { self . force_drop (i) } } } } }
    };
}

impl_210!();