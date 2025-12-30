// Generated macro for get_serializable_bytes_non_empty (function)
macro_rules! Depcrate_varzerovec_componentsget_serializable_bytes_non_empty {
() => {
// Module: crate::varzerovec::components
// Provides: {"get_serializable_bytes_non_empty"}
// Dependencies: {}
# [doc = " Collects the bytes for a VarZeroSlice into a Vec."] # [cfg (feature = "alloc")] pub fn get_serializable_bytes_non_empty < T , A , F > (elements : & [A]) -> Option < alloc :: vec :: Vec < u8 > > where T : VarULE + ? Sized , A : EncodeAsVarULE < T > , F : VarZeroVecFormat , { debug_assert ! (! elements . is_empty ()) ; let len = compute_serializable_len :: < T , A , F > (elements) ? ; debug_assert ! (len >= F :: Len :: SIZE as u32 , "Must have at least F::Len::SIZE bytes to hold the length of the vector") ; let mut output = alloc :: vec ! [0u8 ; len as usize] ; write_serializable_bytes :: < T , A , F > (elements , & mut output) ; Some (output) }
};
}
