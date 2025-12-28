macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! SliceMut {
    () => {
        deps!();
        # [doc = " An indexable representation of a mutable subset of a `RingBuffer`."] pub struct SliceMut < 'a , A , const N : usize > { pub (crate) buffer : & 'a mut RingBuffer < A , N > , pub (crate) range : Range < usize > , }
    };
}

SliceMut!();