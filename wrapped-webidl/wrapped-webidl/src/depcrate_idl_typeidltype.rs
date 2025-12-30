// Generated macro for IdlType (enum)
macro_rules! Depcrate_idl_typeIdlType {
() => {
// Module: crate::idl_type
// Provides: {"IdlType"}
// Dependencies: {}
# [derive (PartialEq , Eq , PartialOrd , Ord , Clone , Debug)] pub (crate) enum IdlType < 'a > { Boolean , Byte , Octet , Short , UnsignedShort , Long , UnsignedLong , LongLong , UnsignedLongLong , Float , UnrestrictedFloat , Double , UnrestrictedDouble , DomString , ByteString , UsvString , Object , Symbol , Error , ArrayBuffer , DataView { allow_shared : bool , } , Int8Array { allow_shared : bool , immutable : bool , } , Uint8Array { allow_shared : bool , immutable : bool , } , Uint8ClampedArray { allow_shared : bool , immutable : bool , } , Int16Array { allow_shared : bool , immutable : bool , } , Uint16Array { allow_shared : bool , immutable : bool , } , Int32Array { allow_shared : bool , immutable : bool , } , Uint32Array { allow_shared : bool , immutable : bool , } , Float32Array { allow_shared : bool , immutable : bool , } , Float64Array { allow_shared : bool , immutable : bool , } , ArrayBufferView { allow_shared : bool , immutable : bool , } , BufferSource { allow_shared : bool , immutable : bool , } , Nullable (Box < IdlType < 'a > >) , FrozenArray (Box < IdlType < 'a > >) , ObservableArray (Box < IdlType < 'a > >) , Sequence (Box < IdlType < 'a > >) , Promise (Box < IdlType < 'a > >) , Record (Box < IdlType < 'a > > , Box < IdlType < 'a > >) , Union (Vec < IdlType < 'a > >) , Any , Undefined , UnknownIdentifier (& 'a str) , Identifier { name : & 'a str , ty : IdentifierType < 'a > , } , }
};
}
