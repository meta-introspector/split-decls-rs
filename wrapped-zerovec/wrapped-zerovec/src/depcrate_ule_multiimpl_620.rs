// Generated macro for impl_620 (impl)
macro_rules! Depcrate_ule_multiimpl_620 {
() => {
// Module: crate::ule::multi
// Provides: {"impl_620"}
// Dependencies: {}
unsafe impl < const LEN : usize , Format : VarZeroVecFormat > VarULE for MultiFieldsULE < LEN , Format > { # [doc = " Note: MultiFieldsULE is usually used in cases where one should be calling .validate_field() directly for"] # [doc = " each field, rather than using the regular VarULE impl."] # [doc = ""] # [doc = " This impl exists so that EncodeAsVarULE can work."] # [inline] fn validate_bytes (slice : & [u8]) -> Result < () , UleError > { < VarZeroLengthlessSlice < [u8] , Format > > :: parse_bytes (LEN as u32 , slice) . map (| _ | ()) } # [inline] unsafe fn from_bytes_unchecked (bytes : & [u8]) -> & Self { mem :: transmute (< VarZeroLengthlessSlice < [u8] , Format > > :: from_bytes_unchecked (bytes)) } }
};
}
