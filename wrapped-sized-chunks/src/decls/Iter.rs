macro_rules! deps {
    () => {
        RawIndex!();
        RingBuffer!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " A reference iterator over a `RingBuffer`."] pub struct Iter < 'a , A , const N : usize > { pub (crate) buffer : & 'a RingBuffer < A , N > , pub (crate) left_index : RawIndex < N > , pub (crate) right_index : RawIndex < N > , pub (crate) remaining : usize , }
    };
}

Iter!()