// Generated macro for WeakArrayMut (struct)
macro_rules! Depcrate_weak_sliceWeakArrayMut {
() => {
// Module: crate::weak_slice
// Provides: {"WeakArrayMut"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct WeakArrayMut < 'a , T , const N : usize > { ptr : * mut [T ; N] , _marker : PhantomData < & 'a mut [T ; N] > , }
};
}
