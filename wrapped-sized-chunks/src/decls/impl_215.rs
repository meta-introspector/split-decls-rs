macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < A , const N : usize > Default for RingBuffer < A , N > { # [inline] # [must_use] fn default () -> Self { Self :: new () } }
    };
}

impl_215!()