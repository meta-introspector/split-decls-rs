// Generated macro for MultiFieldsULE (struct)
macro_rules! Depcrate_ule_multiMultiFieldsULE {
() => {
// Module: crate::ule::multi
// Provides: {"MultiFieldsULE"}
// Dependencies: {}
# [doc = " This type is used by the custom derive to represent multiple [`VarULE`]"] # [doc = " fields packed into a single end-of-struct field. It is not recommended"] # [doc = " to use this type directly, use [`Tuple2VarULE`](crate::ule::tuplevar::Tuple2VarULE) etc instead."] # [doc = ""] # [doc = " Logically, consider it to be `(, , , ..)`"] # [doc = " where `` etc are potentially different [`VarULE`] types."] # [doc = ""] # [doc = " Internally, it is represented by a VarZeroSlice without the length part."] # [derive (PartialEq , Eq)] # [repr (transparent)] pub struct MultiFieldsULE < const LEN : usize , Format : VarZeroVecFormat > (VarZeroLengthlessSlice < [u8] , Format > ,) ;
};
}
