// Generated macro for impl_53 (impl)
macro_rules! Depcrate_authorizationimpl_53 {
() => {
// Module: crate::authorization
// Provides: {"impl_53"}
// Dependencies: {}
impl AuthorizationItem { # [doc = " The required name of the authorization right or environment data."] # [doc = ""] # [doc = " If `name` isn't convertable to a `CString` it will return"] # [doc = " Err(errSecConversionError)."] # [must_use] pub fn name (& self) -> & str { unsafe { CStr :: from_ptr (self . 0 . name) . to_str () . expect ("AuthorizationItem::name failed to convert &str to CStr") } } # [doc = " The information pertaining to the name field. Do not rely on NULL"] # [doc = " termination of string data."] # [inline] # [must_use] pub fn value (& self) -> Option < & [u8] > { if self . 0 . value . is_null () { return None ; } let value = unsafe { std :: slice :: from_raw_parts (self . 0 . value as * const u8 , self . 0 . valueLength) } ; Some (value) } }
};
}
