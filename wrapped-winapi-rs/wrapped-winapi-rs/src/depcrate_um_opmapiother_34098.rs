// Generated macro for other_34098 (other)
macro_rules! Depcrate_um_opmapiother_34098 {
() => {
// Module: crate::um::opmapi
// Provides: {"other_34098"}
// Dependencies: {}
extern "system" { pub fn OPMGetVideoOutputsFromHMONITOR (hMonitor : HMONITOR , vos : OPM_VIDEO_OUTPUT_SEMANTICS , pulNumVideoOutputs : * mut ULONG , pppOPMVideoOutputArray : * mut * mut * mut IOPMVideoOutput ,) -> HRESULT ; pub fn OPMGetVideoOutputForTarget (pAdapterLuid : * mut LUID , VidPnTarget : ULONG , vos : OPM_VIDEO_OUTPUT_SEMANTICS , ppOPMVideoOutput : * mut * mut IOPMVideoOutput ,) -> HRESULT ; pub fn OPMGetVideoOutputsFromIDirect3DDevice9Object (pDirect3DDevice9 : * mut IDirect3DDevice9 , vos : OPM_VIDEO_OUTPUT_SEMANTICS , pulNumVideoOutputs : * mut ULONG , pppOPMVideoOutputArray : * mut * mut * mut IOPMVideoOutput ,) -> HRESULT ; }
};
}
