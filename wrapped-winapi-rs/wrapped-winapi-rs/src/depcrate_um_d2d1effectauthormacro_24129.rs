// Generated macro for macro_24129 (macro)
macro_rules! Depcrate_um_d2d1effectauthormacro_24129 {
() => {
// Module: crate::um::d2d1effectauthor
// Provides: {"macro_24129"}
// Dependencies: {}
RIDL ! { # [uuid (0x13d29038 , 0xc3e6 , 0x4034 , 0x90 , 0x81 , 0x13 , 0xb5 , 0x3a , 0x41 , 0x79 , 0x92)] interface ID2D1TransformGraph (ID2D1TransformGraphVtbl) : IUnknown (IUnknownVtbl) { fn GetInputCount () -> UINT32 , fn SetSingleTransformNode (node : * mut ID2D1TransformNode ,) -> HRESULT , fn AddNode (node : * mut ID2D1TransformNode ,) -> HRESULT , fn RemoveNode (node : * mut ID2D1TransformNode ,) -> HRESULT , fn SetOutputNode (node : * mut ID2D1TransformNode ,) -> HRESULT , fn ConnectNode (fromNode : * mut ID2D1TransformNode , toNode : * mut ID2D1TransformNode , toNodeInputIndex : UINT32 ,) -> HRESULT , fn ConnectToEffectInput (toEffectInputIndex : UINT32 , node : * mut ID2D1TransformNode , toNodeInputIndex : UINT32 ,) -> HRESULT , fn Clear () -> () , fn SetPassthroughGraph (effectInputIndex : UINT32 ,) -> HRESULT , } }
};
}
