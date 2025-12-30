// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > EventVisitor < 'a > { fn new (buf : & 'a mut Vec < u8 > , prefix : Option < & 'a str >) -> Self { Self { buf , prefix } } fn put_prefix (& mut self , field : & Field) { if let Some (prefix) = self . prefix { if field . name () != "message" { self . buf . extend_from_slice (prefix . as_bytes ()) ; self . buf . push (b'_') ; } } } }
};
}
