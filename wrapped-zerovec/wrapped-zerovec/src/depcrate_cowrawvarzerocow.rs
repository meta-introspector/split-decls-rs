// Generated macro for RawVarZeroCow (struct)
macro_rules! Depcrate_cowRawVarZeroCow {
() => {
// Module: crate::cow
// Provides: {"RawVarZeroCow"}
// Dependencies: {}
# [doc = " VarZeroCow without the `V` to simulate a dropck eyepatch"] # [doc = " (i.e., prove to rustc that the dtor is not able to observe V or 'a)"] # [doc = ""] # [doc = " This is effectively `Cow<'a, [u8]>`, with the lifetime managed externally"] struct RawVarZeroCow { # [doc = " Pointer to data"] # [doc = ""] # [doc = " # Safety Invariants"] # [doc = ""] # [doc = " 1. This slice must always be valid as a byte slice"] # [doc = " 2. If `owned` is true, this slice can be freed."] # [doc = " 3. VarZeroCow, the only user of this type, will impose an additional invariant that the buffer is a valid V"] buf : NonNull < [u8] > , # [doc = " The buffer is `Box<[u8]>` if true"] # [cfg (feature = "alloc")] owned : bool , }
};
}
