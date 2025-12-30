// Generated macro for TYPEDEF_CA (macro)
macro_rules! Depcrate_um_propidlTYPEDEF_CA {
() => {
// Module: crate::um::propidl
// Provides: {"TYPEDEF_CA"}
// Dependencies: {}
macro_rules ! TYPEDEF_CA { ($ type_ : ty , $ name : ident) => { STRUCT ! { struct $ name { cElems : $ crate :: shared :: ntdef :: ULONG , pElems : * mut $ type_ , } } } }
};
}
