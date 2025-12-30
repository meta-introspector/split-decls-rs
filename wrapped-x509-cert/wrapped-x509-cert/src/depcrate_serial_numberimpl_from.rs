// Generated macro for impl_from (macro)
macro_rules! Depcrate_serial_numberimpl_from {
() => {
// Module: crate::serial_number
// Provides: {"impl_from"}
// Dependencies: {}
macro_rules ! impl_from { ($ source : ty) => { impl From <$ source > for SerialNumber { fn from (inner : $ source) -> SerialNumber { let serial_number = & inner . to_be_bytes () [..] ; let serial_number = asn1 :: Uint :: new (serial_number) . unwrap () ; SerialNumber :: new (serial_number . as_bytes ()) . unwrap () } } } ; }
};
}
