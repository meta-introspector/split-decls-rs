// Generated macro for macro_34605 (macro)
macro_rules! Depcrate_um_propidlmacro_34605 {
() => {
// Module: crate::um::propidl
// Provides: {"macro_34605"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000013A , 0x0000 , 0x0000 , 0xC0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IPropertySetStorage (IPropertySetStorageVtbl) : IUnknown (IUnknownVtbl) { fn Create (rfmtid : REFFMTID , pclsid : * const CLSID , grfFlags : DWORD , grfMode : DWORD , ppprstg : * mut * mut IPropertyStorage ,) -> HRESULT , fn Open (rfmtid : REFFMTID , grfMode : DWORD , ppprstg : * mut * mut IPropertyStorage ,) -> HRESULT , fn Delete (rfmtid : REFFMTID ,) -> HRESULT , fn Enum (ppenum : * mut * mut IEnumSTATPROPSTG ,) -> HRESULT , } }
};
}
