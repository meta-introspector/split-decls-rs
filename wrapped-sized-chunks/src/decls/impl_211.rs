macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < A , const N : usize > HasLength for RingBuffer < A , N > { # [doc = " Get the length of the ring buffer."] # [inline] # [must_use] fn len (& self) -> usize { self . length } }
    };
}

impl_211!()