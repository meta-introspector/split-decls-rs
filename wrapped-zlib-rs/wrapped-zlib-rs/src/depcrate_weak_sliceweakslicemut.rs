// Generated macro for WeakSliceMut (struct)
macro_rules! Depcrate_weak_sliceWeakSliceMut {
() => {
// Module: crate::weak_slice
// Provides: {"WeakSliceMut"}
// Dependencies: {}
# [doc = " a mutable \"slice\" (bundle of pointer and length). The main goal of this type is passing MIRI"] # [doc = " with stacked borrows. In particular, storing a standard slice in data structures violates the"] # [doc = " stacked borrows rule when that slice is deallocated. By only materializing the slice when"] # [doc = " needed for data access, hence bounding the lifetime more tightly, this restriction is circumvented."] # [derive (Debug)] pub (crate) struct WeakSliceMut < 'a , T > { ptr : * mut T , len : usize , _marker : PhantomData < & 'a mut [T] > , }
};
}
