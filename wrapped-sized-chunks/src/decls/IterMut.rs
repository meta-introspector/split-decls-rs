macro_rules! deps {
    () => {
        RawIndex!();
        RingBuffer!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " A mutable reference iterator over a `RingBuffer`."] pub struct IterMut < 'a , A , const N : usize > { data : * mut A , left_index : RawIndex < N > , right_index : RawIndex < N > , remaining : usize , phantom : PhantomData < & 'a () > , }
    };
}

IterMut!();