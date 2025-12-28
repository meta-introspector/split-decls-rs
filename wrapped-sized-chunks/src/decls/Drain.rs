macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! Drain {
    () => {
        deps!();
        # [doc = " A draining iterator over a `RingBuffer`."] pub struct Drain < 'a , A , const N : usize > { pub (crate) buffer : & 'a mut RingBuffer < A , N > , }
    };
}

Drain!();