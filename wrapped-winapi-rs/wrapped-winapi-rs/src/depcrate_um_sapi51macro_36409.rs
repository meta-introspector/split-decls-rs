// Generated macro for macro_36409 (macro)
macro_rules! Depcrate_um_sapi51macro_36409 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36409"}
// Dependencies: {}
RIDL ! { # [uuid (0x93384e18 , 0x5014 , 0x43d5 , 0xad , 0xbb , 0xa7 , 0x8e , 0x05 , 0x59 , 0x26 , 0xbd)] interface ISpResourceManager (ISpResourceManagerVtbl) : IServiceProvider (IServiceProviderVtbl) { fn SetObject (guidServiceId : REFGUID , pUnkObject : * mut IUnknown ,) -> HRESULT , fn GetObject (guidServiceId : REFGUID , ObjectCLSID : REFCLSID , ObjectIID : REFIID , fReleaseWhenLastExternalRefReleased : BOOL , ppObject : * mut * mut c_void ,) -> HRESULT , } }
};
}
