// Generated macro for VarZeroLengthlessSlice (struct)
macro_rules! Depcrate_varzerovec_lengthlessVarZeroLengthlessSlice {
() => {
// Module: crate::varzerovec::lengthless
// Provides: {"VarZeroLengthlessSlice"}
// Dependencies: {}
# [doc = " A slice representing the index and data tables of a VarZeroVec,"] # [doc = " *without* any length fields. The length field is expected to be stored elsewhere."] # [doc = ""] # [doc = " Without knowing the length this is of course unsafe to use directly."] # [repr (transparent)] # [derive (PartialEq , Eq)] pub (crate) struct VarZeroLengthlessSlice < T : ? Sized , F > { marker : PhantomData < (F , T) > , # [doc = " The original slice this was constructed from"] entire_slice : [u8] , }
};
}
