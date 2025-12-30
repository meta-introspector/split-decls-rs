// Generated macro for VarZeroCow (struct)
macro_rules! Depcrate_cowVarZeroCow {
() => {
// Module: crate::cow
// Provides: {"VarZeroCow"}
// Dependencies: {}
# [doc = " Copy-on-write type that efficiently represents [`VarULE`] types as their bitstream representation."] # [doc = ""] # [doc = " The primary use case for [`VarULE`] types is the ability to store complex variable-length datastructures"] # [doc = " inside variable-length collections like [`crate::VarZeroVec`]."] # [doc = ""] # [doc = " Underlying this ability is the fact that [`VarULE`] types can be efficiently represented as a flat"] # [doc = " bytestream."] # [doc = ""] # [doc = " In zero-copy cases, sometimes one wishes to unconditionally use this bytestream representation, for example"] # [doc = " to save stack size. A struct with five `Cow<'a, str>`s is not as stack-efficient as a single `Cow` containing"] # [doc = " the bytestream representation of, say, `Tuple5VarULE<str, str, str, str, str>`."] # [doc = ""] # [doc = " This type helps in this case: It is logically a `Cow<'a, V>`, with some optimizations, that is guaranteed"] # [doc = " to serialize as a byte stream in machine-readable scenarios."] # [doc = ""] # [doc = " During human-readable serialization, it will fall back to the serde impls on `V`, which ought to have"] # [doc = " a human-readable variant."] pub struct VarZeroCow < 'a , V : ? Sized > { # [doc = " Safety invariant: Contained slice must be a valid V"] # [doc = " It may or may not have a lifetime valid for 'a, it must be valid for as long as this type is around."] raw : RawVarZeroCow , marker1 : PhantomData < & 'a V > , # [cfg (feature = "alloc")] marker2 : PhantomData < Box < V > > , }
};
}
