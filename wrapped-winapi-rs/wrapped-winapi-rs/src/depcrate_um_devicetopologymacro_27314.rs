// Generated macro for macro_27314 (macro)
macro_rules! Depcrate_um_devicetopologymacro_27314 {
() => {
// Module: crate::um::devicetopology
// Provides: {"macro_27314"}
// Dependencies: {}
RIDL ! { # [uuid (0x28f54685 , 0x06fd , 0x11d2 , 0xb2 , 0x7a , 0x00 , 0xa0 , 0xc9 , 0x22 , 0x31 , 0x96)] interface IKsControl (IKsControlVtbl) : IUnknown (IUnknownVtbl) { fn KsProperty (Property : PKSPROPERTY , PropertyLength : ULONG , PropertyData : * mut c_void , DataLength : ULONG , BytesReturned : * mut ULONG ,) -> HRESULT , fn KsMethod (Method : PKSMETHOD , MethodLength : ULONG , MethodData : * mut c_void , DataLength : ULONG , BytesReturned : * mut ULONG ,) -> HRESULT , fn KsEvent (Event : PKSEVENT , EventLength : ULONG , EventData : * mut c_void , DataLength : ULONG , BytesReturned : * mut ULONG ,) -> HRESULT , } }
};
}
