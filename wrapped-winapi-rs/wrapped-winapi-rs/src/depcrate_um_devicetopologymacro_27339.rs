// Generated macro for macro_27339 (macro)
macro_rules! Depcrate_um_devicetopologymacro_27339 {
() => {
// Module: crate::um::devicetopology
// Provides: {"macro_27339"}
// Dependencies: {}
RIDL ! { # [uuid (0x2a07407e , 0x6497 , 0x4a18 , 0x97 , 0x87 , 0x32 , 0xf7 , 0x9b , 0xd0 , 0xd9 , 0x8f)] interface IDeviceTopology (IDeviceTopologyVtbl) : IUnknown (IUnknownVtbl) { fn GetConnectorCount (pCount : * mut UINT ,) -> HRESULT , fn GetConnector (nIndex : UINT , ppConnector : * mut * mut IConnector ,) -> HRESULT , fn GetSubunitCount (pCount : * mut UINT ,) -> HRESULT , fn GetSubunit (nIndex : UINT , ppSubunit : * mut * mut ISubunit ,) -> HRESULT , fn GetPartById (nId : UINT , ppPart : * mut * mut IPart ,) -> HRESULT , fn GetDeviceId (ppwstrDeviceId : * mut LPWSTR ,) -> HRESULT , fn GetSignalPath (pIPartFrom : * mut IPart , pIPartTo : * mut IPart , bRejectMixedPaths : BOOL , ppParts : * mut * mut IPartsList ,) -> HRESULT , } }
};
}
