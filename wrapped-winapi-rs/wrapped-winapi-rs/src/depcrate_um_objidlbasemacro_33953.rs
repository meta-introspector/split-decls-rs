// Generated macro for macro_33953 (macro)
macro_rules! Depcrate_um_objidlbasemacro_33953 {
() => {
// Module: crate::um::objidlbase
// Provides: {"macro_33953"}
// Dependencies: {}
RIDL ! { # [uuid (0x000001c1 , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IEnumContextProps (IEnumContextPropsVtbl) : IUnknown (IUnknownVtbl) { fn Next (celt : ULONG , pContextProperties : * mut ContextProperty , pceltFetched : * mut ULONG ,) -> HRESULT , fn Skip (celt : ULONG ,) -> HRESULT , fn Reset () -> HRESULT , fn Clone (ppEnumContextProps : * mut * mut IEnumContextProps ,) -> HRESULT , fn Count (pcelt : * mut ULONG ,) -> HRESULT , } }
};
}
