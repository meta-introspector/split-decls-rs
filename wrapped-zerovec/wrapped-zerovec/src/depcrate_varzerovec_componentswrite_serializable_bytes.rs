// Generated macro for write_serializable_bytes (function)
macro_rules! Depcrate_varzerovec_componentswrite_serializable_bytes {
() => {
// Module: crate::varzerovec::components
// Provides: {"write_serializable_bytes"}
// Dependencies: {}
# [doc = " Writes the bytes for a VarZeroSlice into an output buffer."] # [doc = ""] # [doc = " Every byte in the buffer will be initialized after calling this function."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the buffer is not exactly the correct length."] pub fn write_serializable_bytes < T , A , F > (elements : & [A] , output : & mut [u8]) where T : VarULE + ? Sized , A : EncodeAsVarULE < T > , F : VarZeroVecFormat , { if elements . is_empty () { return ; } assert ! (elements . len () <= F :: Len :: MAX_VALUE as usize) ; # [expect (clippy :: expect_used)] let num_elements_ule = F :: Len :: iule_from_usize (elements . len ()) . expect (F :: Len :: TOO_LARGE_ERROR) ; # [expect (clippy :: indexing_slicing)] output [0 .. F :: Len :: SIZE] . copy_from_slice (ULE :: slice_as_bytes (& [num_elements_ule])) ; # [expect (clippy :: indexing_slicing)] write_serializable_bytes_without_length :: < T , A , F > (elements , & mut output [F :: Len :: SIZE ..]) ; }
};
}
