// Generated macro for macro_33847 (macro)
macro_rules! Depcrate_um_objidlmacro_33847 {
() => {
// Module: crate::um::objidl
// Provides: {"macro_33847"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000010f , 0x0000 , 0x0000 , 0xc0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IAdviseSink (IAdviseSinkVtbl) : IUnknown (IUnknownVtbl) { fn OnDataChange (pformatetc : * mut FORMATETC , pStgmed : * mut STGMEDIUM ,) -> c_void , fn OnViewChange (dwAspect : DWORD , lindex : LONG ,) -> c_void , fn OnRename (pmk : * mut IMoniker ,) -> c_void , fn OnSave () -> c_void , fn OnClose () -> c_void , } }
};
}
