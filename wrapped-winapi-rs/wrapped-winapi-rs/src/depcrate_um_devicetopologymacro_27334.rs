// Generated macro for macro_27334 (macro)
macro_rules! Depcrate_um_devicetopologymacro_27334 {
() => {
// Module: crate::um::devicetopology
// Provides: {"macro_27334"}
// Dependencies: {}
RIDL ! { # [uuid (0xae2de0e4 , 0x5bca , 0x4f2d , 0xaa , 0x46 , 0x5d , 0x13 , 0xf8 , 0xfd , 0xb3 , 0xa9)] interface IPart (IPartVtbl) : IUnknown (IUnknownVtbl) { fn GetName (ppwstrName : * mut LPWSTR ,) -> HRESULT , fn GetLocalId (pnId : * mut UINT ,) -> HRESULT , fn GetGlobalId (ppwstrGlobalId : * mut LPWSTR ,) -> HRESULT , fn GetPartType (pPartType : * mut PartType ,) -> HRESULT , fn GetSubType (pSubType : * mut GUID ,) -> HRESULT , fn GetControlInterfaceCount (pCount : * mut UINT ,) -> HRESULT , fn GetControlInterface (nIndex : UINT , ppInterfaceDesc : * mut * mut IControlInterface ,) -> HRESULT , fn EnumPartsIncoming (ppParts : * mut * mut IPartsList ,) -> HRESULT , fn EnumPartsOutgoing (ppParts : * mut * mut IPartsList ,) -> HRESULT , fn GetTopologyObject (ppTopology : * mut * mut IDeviceTopology ,) -> HRESULT , fn Activate (dwClsContext : DWORD , refiid : REFIID , ppvObject : * mut * mut c_void ,) -> HRESULT , fn RegisterControlChangeCallback (riid : REFGUID , pNotify : * mut IControlChangeNotify ,) -> HRESULT , fn UnregisterControlChangeCallback (pNotify : * mut IControlChangeNotify ,) -> HRESULT , } }
};
}
