macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! OwnedIter {
    () => {
        deps!();
        # [doc = " A consuming iterator over a `RingBuffer`."] pub struct OwnedIter < A , const N : usize > { pub (crate) buffer : RingBuffer < A , N > , }
    };
}

OwnedIter!()