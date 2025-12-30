// Generated macro for macro_39380 (macro)
macro_rules! Depcrate_um_vsbackupmacro_39380 {
() => {
// Module: crate::um::vsbackup
// Provides: {"macro_39380"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000000 , 0x0000 , 0x0000 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00)] interface IVssWMComponent (IVssWMComponentVtbl) : IUnknown (IUnknownVtbl) { fn GetComponentInfo (ppInfo : * mut PVSSCOMPONENTINFO ,) -> HRESULT , fn FreeComponentInfo (pInfo : PVSSCOMPONENTINFO ,) -> HRESULT , fn GetFile (iFile : UINT , ppFiledesc : * mut * mut IVssWMFiledesc ,) -> HRESULT , fn GetDatabaseFile (iDBFile : UINT , ppFiledesc : * mut * mut IVssWMFiledesc ,) -> HRESULT , fn GetDatabaseLogFile (iDbLogFile : UINT , ppFiledesc : * mut * mut IVssWMFiledesc ,) -> HRESULT , fn GetDependency (iDependency : UINT , ppDependency : * mut * mut IVssWMDependency ,) -> HRESULT , } }
};
}
