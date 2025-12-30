// Generated macro for macro_37103 (macro)
macro_rules! Depcrate_um_servprovmacro_37103 {
() => {
// Module: crate::um::servprov
// Provides: {"macro_37103"}
// Dependencies: {}
RIDL ! { # [uuid (0x6d5140c1 , 0x7436 , 0x11ce , 0x80 , 0x34 , 0x00 , 0xaa , 0x00 , 0x60 , 0x09 , 0xfa)] interface IServiceProvider (IServiceProviderVtbl) : IUnknown (IUnknownVtbl) { fn QueryService (guidService : REFGUID , riid : REFIID , ppvObject : * mut * mut c_void ,) -> HRESULT , fn RemoteQueryService (guidService : REFGUID , riid : REFIID , ppvObject : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
