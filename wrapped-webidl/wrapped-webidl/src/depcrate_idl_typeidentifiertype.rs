// Generated macro for IdentifierType (enum)
macro_rules! Depcrate_idl_typeIdentifierType {
() => {
// Module: crate::idl_type
// Provides: {"IdentifierType"}
// Dependencies: {}
# [derive (PartialEq , Eq , PartialOrd , Ord , Clone , Debug)] pub (crate) enum IdentifierType < 'a > { Callback , Iterator , AsyncIterator , Interface (& 'a str) , Dictionary (& 'a str) , Enum (& 'a str) , CallbackInterface { name : & 'a str , single_function : bool , } , UnsignedLongLong , AllowSharedBufferSource { immutable : bool , } , Int8Slice { allow_shared : bool , immutable : bool , } , Uint8Slice { allow_shared : bool , immutable : bool , } , Uint8ClampedSlice { allow_shared : bool , immutable : bool , } , Int16Slice { allow_shared : bool , immutable : bool , } , Uint16Slice { allow_shared : bool , immutable : bool , } , Int32Slice { allow_shared : bool , immutable : bool , } , Uint32Slice { allow_shared : bool , immutable : bool , } , Float32Slice { allow_shared : bool , immutable : bool , } , Float64Slice { allow_shared : bool , immutable : bool , } , }
};
}
