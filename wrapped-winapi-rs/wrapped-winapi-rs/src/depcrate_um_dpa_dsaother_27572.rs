// Generated macro for other_27572 (other)
macro_rules! Depcrate_um_dpa_dsaother_27572 {
() => {
// Module: crate::um::dpa_dsa
// Provides: {"other_27572"}
// Dependencies: {}
extern "system" { pub fn DPA_LoadStream (phdpa : * mut HDPA , pfn : PFNDPASTREAM , pstream : * mut IStream , pvInstData : * mut c_void ,) -> HRESULT ; pub fn DPA_SaveStream (hdpa : HDPA , pfn : PFNDPASTREAM , pstream : * mut IStream , pvInstData : * mut c_void ,) -> HRESULT ; }
};
}
