// Generated macro for macro_39590 (macro)
macro_rules! Depcrate_um_wbemclimacro_39590 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39590"}
// Dependencies: {}
RIDL ! { # [uuid (0x49353c9a , 0x516b , 0x11d1 , 0xae , 0xa6 , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemObjectAccess (IWbemObjectAccessVtbl) : IWbemClassObject (IWbemClassObjectVtbl) { fn GetPropertyHandle (wszPropertyName : LPCWSTR , pType : * mut CIMTYPE , plHandle : * mut c_long ,) -> HRESULT , fn WritePropertyValue (lHandle : c_long , lNumBytes : c_long , aData : * const byte ,) -> HRESULT , fn ReadPropertyValue (lHandle : c_long , lBufferSize : c_long , plNumBytes : * mut c_long , aData : * mut byte ,) -> HRESULT , fn ReadDWORD (lHandle : c_long , pdw : * mut DWORD ,) -> HRESULT , fn WriteDWORD (lHandle : c_long , dw : DWORD ,) -> HRESULT , fn ReadQWORD (lHandle : c_long , pqw : * mut __uint64 ,) -> HRESULT , fn WriteQWORD (lHandle : c_long , pw : __uint64 ,) -> HRESULT , fn GetPropertyInfoByHandle (lHandle : c_long , pstrName : * mut BSTR , pType : * mut CIMTYPE ,) -> HRESULT , fn Lock (lFlags : c_long ,) -> HRESULT , fn Unlock (lFlags : c_long ,) -> HRESULT , } }
};
}
