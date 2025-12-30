// Generated macro for IterMut (struct)
macro_rules! Depcrate_ring_buffer_iterIterMut {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable reference iterator over a `RingBuffer`."] pub struct IterMut < 'a , A , const N : usize > { data : * mut A , left_index : RawIndex < N > , right_index : RawIndex < N > , remaining : usize , phantom : PhantomData < & 'a () > , }
};
}
