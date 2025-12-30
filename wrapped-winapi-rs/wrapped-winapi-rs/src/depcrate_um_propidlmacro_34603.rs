// Generated macro for macro_34603 (macro)
macro_rules! Depcrate_um_propidlmacro_34603 {
() => {
// Module: crate::um::propidl
// Provides: {"macro_34603"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000138 , 0x0000 , 0x0000 , 0xC0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IPropertyStorage (IPropertyStorageVtbl) : IUnknown (IUnknownVtbl) { fn ReadMultiple (cpspec : ULONG , rgpspec : * const PROPSPEC , rgpropvar : * mut PROPVARIANT ,) -> HRESULT , fn WriteMultiple (cpspec : ULONG , rgpspec : * const PROPSPEC , rgpropvar : * const PROPVARIANT ,) -> HRESULT , fn DeleteMultiple (cpspec : ULONG , rgpspec : * const PROPSPEC ,) -> HRESULT , fn ReadPropertyNames (cppropid : ULONG , rgpropid : * const PROPID , rglpwstrName : * mut LPOLESTR ,) -> HRESULT , fn WritePropertyNames (cppropid : ULONG , rgpropid : * const PROPID , rglpwstrName : * const LPOLESTR ,) -> HRESULT , fn DeletePropertyNames (cppropid : ULONG , rgpropid : * const PROPID ,) -> HRESULT , fn Commit (grfCommitFlags : DWORD ,) -> HRESULT , fn Revert () -> HRESULT , fn Enum (ppenum : * mut * mut IEnumSTATPROPSTG ,) -> HRESULT , fn SetTimes (pctime : * const FILETIME , patime : * const FILETIME , pmtime : * const FILETIME ,) -> HRESULT , fn SetClass (clsid : REFCLSID ,) -> HRESULT , fn Stat (pstatpsstg : * mut STATPROPSETSTG ,) -> HRESULT , } }
};
}
