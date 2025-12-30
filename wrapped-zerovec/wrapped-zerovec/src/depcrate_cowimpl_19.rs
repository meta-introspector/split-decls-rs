// Generated macro for impl_19 (impl)
macro_rules! Depcrate_cowimpl_19 {
() => {
// Module: crate::cow
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a , V : ? Sized > VarZeroCow < 'a , V > { # [doc = " Whether or not this is owned"] pub fn is_owned (& self) -> bool { self . raw . is_owned () } # [doc = " Get the byte representation of this type"] # [doc = ""] # [doc = " Is also always a valid `V` and can be passed to"] # [doc = " `V::from_bytes_unchecked()`"] pub fn as_bytes (& self) -> & [u8] { self . raw . as_bytes () } # [doc = " Invariant: `raw` must wrap a valid V, either owned or borrowed for 'a"] const unsafe fn from_raw (raw : RawVarZeroCow) -> Self { Self { raw , marker1 : PhantomData , # [cfg (feature = "alloc")] marker2 : PhantomData , } } }
};
}
