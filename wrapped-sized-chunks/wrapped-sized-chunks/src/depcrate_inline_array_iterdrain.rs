// Generated macro for Drain (struct)
macro_rules! Depcrate_inline_array_iterDrain {
() => {
// Module: crate::inline_array::iter
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the elements of an `InlineArray`."] # [doc = ""] # [doc = " \"Draining\" means that as the iterator yields each element, it's removed from"] # [doc = " the `InlineArray`. When the iterator terminates, the array will be empty."] # [doc = " This is different from the consuming iterator `Iter` in that `Iter` will"] # [doc = " take ownership of the `InlineArray` and discard it when you're done"] # [doc = " iterating, while `Drain` leaves you still owning the drained `InlineArray`."] pub struct Drain < 'a , A , T > { pub (crate) array : & 'a mut InlineArray < A , T > , }
};
}
