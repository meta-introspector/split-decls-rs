// Generated macro for macro_34399 (macro)
macro_rules! Depcrate_um_portabledevicetypesmacro_34399 {
() => {
// Module: crate::um::portabledevicetypes
// Provides: {"macro_34399"}
// Dependencies: {}
RIDL ! { # [uuid (0x89b2e422 , 0x4f1b , 0x4316 , 0xbc , 0xef , 0xa4 , 0x4a , 0xfe , 0xa8 , 0x3e , 0xb3)] interface IPortableDevicePropVariantCollection (IPortableDevicePropVariantCollectionVtbl) : IUnknown (IUnknownVtbl) { fn GetCount (pcElems : * mut DWORD ,) -> HRESULT , fn GetAt (dwIndex : DWORD , pValue : * mut PROPVARIANT ,) -> HRESULT , fn Add (pValue : * const PROPVARIANT ,) -> HRESULT , fn GetType (pvt : * mut VARTYPE ,) -> HRESULT , fn ChangeType (vt : VARTYPE ,) -> HRESULT , fn Clear () -> HRESULT , fn RemoveAt (dwIndex : DWORD ,) -> HRESULT , } }
};
}
