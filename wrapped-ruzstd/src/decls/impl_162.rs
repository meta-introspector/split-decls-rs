macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl Drop for RingBuffer { fn drop (& mut self) { if self . cap == 0 { return ; } let current_layout = unsafe { Layout :: array :: < u8 > (self . cap) . unwrap_unchecked () } ; unsafe { dealloc (self . buf . as_ptr () , current_layout) ; } } }
    };
}

impl_162!();