macro_rules! monotonic {
    () => {
        mod monotonic { use std :: ops :: { Deref , DerefMut } ; # [doc = " A `MonotonicVec` is a `Vec` which can only be grown."] # [doc = " Once inserted, an element can never be removed or swapped,"] # [doc = " guaranteeing that any indices into a `MonotonicVec` are stable"] pub struct MonotonicVec < T > (Vec < T >) ; impl < T > MonotonicVec < T > { pub (super) fn push (& mut self , val : T) { self . 0 . push (val) ; } } impl < T > Default for MonotonicVec < T > { fn default () -> Self { MonotonicVec (vec ! []) } } impl < T > Deref for MonotonicVec < T > { type Target = Vec < T > ; fn deref (& self) -> & Self :: Target { & self . 0 } } impl < T > ! DerefMut for MonotonicVec < T > { } }
    };
}

monotonic!()