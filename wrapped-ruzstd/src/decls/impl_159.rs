macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        unsafe impl Send for RingBuffer { }
    };
}

impl_159!();