// Generated macro for macro_34398 (macro)
macro_rules! Depcrate_um_portabledevicetypesmacro_34398 {
() => {
// Module: crate::um::portabledevicetypes
// Provides: {"macro_34398"}
// Dependencies: {}
RIDL ! { # [uuid (0xdada2357 , 0xe0ad , 0x492e , 0x98 , 0xdb , 0xdd , 0x61 , 0xc5 , 0x3b , 0xa3 , 0x53)] interface IPortableDeviceKeyCollection (IPortableDeviceKeyCollectionVtbl) : IUnknown (IUnknownVtbl) { fn GetCount (pcElems : * mut DWORD ,) -> HRESULT , fn GetAt (dwIndex : DWORD , pKey : * mut PROPERTYKEY ,) -> HRESULT , fn Add (Key : REFPROPERTYKEY ,) -> HRESULT , fn Clear () -> HRESULT , fn RemoveAt (dwIndex : DWORD ,) -> HRESULT , } }
};
}
