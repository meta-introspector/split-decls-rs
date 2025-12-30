// Generated macro for impl_167 (impl)
macro_rules! Depcrate_idl_typeimpl_167 {
() => {
// Module: crate::idl_type
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'a > ToIdlType < 'a > for Identifier < 'a > { fn to_idl_type (& self , record : & FirstPassRecord < 'a >) -> IdlType < 'a > { let ty = if self . 0 == "DOMTimeStamp" { IdentifierType :: UnsignedLongLong } else if self . 0 == "AllowSharedBufferSource" { IdentifierType :: AllowSharedBufferSource { immutable : false } } else if let Some (idl_type) = record . typedefs . get (& self . 0) { return idl_type . to_idl_type (record) ; } else if record . interfaces . contains_key (self . 0) { IdentifierType :: Interface (self . 0) } else if record . dictionaries . contains_key (self . 0) { IdentifierType :: Dictionary (self . 0) } else if record . enums . contains_key (self . 0) { IdentifierType :: Enum (self . 0) } else if record . callbacks . contains (self . 0) { IdentifierType :: Callback } else if record . iterators . contains (self . 0) { IdentifierType :: Iterator } else if record . async_iterators . contains (self . 0) { IdentifierType :: AsyncIterator } else if let Some (data) = record . callback_interfaces . get (self . 0) { IdentifierType :: CallbackInterface { name : self . 0 , single_function : data . single_function , } } else if self . 0 == "WindowProxy" { IdentifierType :: Interface ("Window") } else { log :: warn ! ("Unrecognized type: {}" , self . 0) ; return IdlType :: UnknownIdentifier (self . 0) ; } ; IdlType :: id (self . 0 , ty) } }
};
}
