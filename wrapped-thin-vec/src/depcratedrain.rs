// Generated macro for Drain (struct)
macro_rules! DepcrateDrain {
() => {
// Module: crate
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator for `ThinVec<T>`."] # [doc = ""] # [doc = " This `struct` is created by [`ThinVec::drain`]."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use thin_vec::thin_vec;"] # [doc = ""] # [doc = " let mut v = thin_vec![0, 1, 2];"] # [doc = " let iter: thin_vec::Drain<_> = v.drain(..);"] # [doc = " ```"] pub struct Drain < 'a , T > { # [doc = " An iterator over the elements we're removing."] # [doc = ""] # [doc = " As we go we'll be `read`ing out of the shared refs yielded by this."] # [doc = " It's ok to use Iter here because it promises to only take refs to the parts"] # [doc = " we haven't yielded yet."] iter : Iter < 'a , T > , # [doc = " The actual ThinVec, which we need to hold onto to undo the leak amplification"] # [doc = " and backshift the tail into place. This should only be accessed when we're"] # [doc = " completely done with the Iter in the `drop` impl of this type (or miri will get mad)."] # [doc = ""] # [doc = " Since we set the `len` of this to be before `Iter`, we can use that `len`"] # [doc = " to retrieve the index of the start of the drain range later."] vec : NonNull < ThinVec < T > > , # [doc = " The one-past-the-end index of the drain range, or equivalently the start of the tail."] end : usize , # [doc = " The length of the tail."] tail : usize , }
};
}
