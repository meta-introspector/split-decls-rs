// Generated macro for macro_34400 (macro)
macro_rules! Depcrate_um_portabledevicetypesmacro_34400 {
() => {
// Module: crate::um::portabledevicetypes
// Provides: {"macro_34400"}
// Dependencies: {}
RIDL ! { # [uuid (0x6e3f2d79 , 0x4e07 , 0x48c4 , 0x82 , 0x08 , 0xd8 , 0xc2 , 0xe5 , 0xaf , 0x4a , 0x99)] interface IPortableDeviceValuesCollection (IPortableDeviceValuesCollectionVtbl) : IUnknown (IUnknownVtbl) { fn GetCount (pcElems : * mut DWORD ,) -> HRESULT , fn GetAt (dwIndex : DWORD , ppValues : * mut * mut IPortableDeviceValues ,) -> HRESULT , fn Add (pValues : * mut IPortableDeviceValues ,) -> HRESULT , fn Clear () -> HRESULT , fn RemoveAt (dwIndex : DWORD ,) -> HRESULT , } }
};
}
