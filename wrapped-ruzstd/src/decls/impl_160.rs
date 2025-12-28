macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        unsafe impl Sync for RingBuffer { }
    };
}

impl_160!()