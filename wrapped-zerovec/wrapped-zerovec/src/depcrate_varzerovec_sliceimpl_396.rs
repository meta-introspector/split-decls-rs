// Generated macro for impl_396 (impl)
macro_rules! Depcrate_varzerovec_sliceimpl_396 {
() => {
// Module: crate::varzerovec::slice
// Provides: {"impl_396"}
// Dependencies: {}
unsafe impl < T : VarULE + ? Sized + 'static , F : VarZeroVecFormat > VarULE for VarZeroSlice < T , F > { fn validate_bytes (bytes : & [u8]) -> Result < () , UleError > { let _ : VarZeroVecComponents < T , F > = VarZeroVecComponents :: parse_bytes (bytes) . map_err (| _ | UleError :: parse :: < Self > ()) ? ; Ok (()) } unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { mem :: transmute (bytes) } fn as_bytes (& self) -> & [u8] { & self . entire_slice } }
};
}
