macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! Slice {
    () => {
        deps!();
        # [doc = " An indexable representation of a subset of a `RingBuffer`."] pub struct Slice < 'a , A , const N : usize > { pub (crate) buffer : & 'a RingBuffer < A , N > , pub (crate) range : Range < usize > , }
    };
}

Slice!();