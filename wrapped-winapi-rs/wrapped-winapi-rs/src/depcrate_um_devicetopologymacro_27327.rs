// Generated macro for macro_27327 (macro)
macro_rules! Depcrate_um_devicetopologymacro_27327 {
() => {
// Module: crate::um::devicetopology
// Provides: {"macro_27327"}
// Dependencies: {}
RIDL ! { # [uuid (0x3b22bcbf , 0x2586 , 0x4af0 , 0x85 , 0x83 , 0x20 , 0x5d , 0x39 , 0x1b , 0x80 , 0x7c)] interface IDeviceSpecificProperty (IDeviceSpecificPropertyVtbl) : IUnknown (IUnknownVtbl) { fn GetType (pVType : * mut VARTYPE ,) -> HRESULT , fn GetValue (pvValue : * mut c_void , pcbValue : * mut DWORD ,) -> HRESULT , fn SetValue (pvValue : * mut c_void , cbValue : DWORD , pguidEventContext : LPCGUID ,) -> HRESULT , fn Get4BRange (plMin : * mut LONG , plMax : * mut LONG , plStepping : * mut LONG ,) -> HRESULT , } }
};
}
