// Generated macro for macro_32022 (macro)
macro_rules! Depcrate_um_mmdeviceapimacro_32022 {
() => {
// Module: crate::um::mmdeviceapi
// Provides: {"macro_32022"}
// Dependencies: {}
RIDL ! { # [uuid (0x0bd7a1be , 0x7a1a , 0x44db , 0x83 , 0x97 , 0xcc , 0x53 , 0x92 , 0x38 , 0x7b , 0x5e)] interface IMMDeviceCollection (IMMDeviceCollectionVtbl) : IUnknown (IUnknownVtbl) { fn GetCount (pcDevices : * const UINT ,) -> HRESULT , fn Item (nDevice : UINT , ppDevice : * mut * mut IMMDevice ,) -> HRESULT , } }
};
}
