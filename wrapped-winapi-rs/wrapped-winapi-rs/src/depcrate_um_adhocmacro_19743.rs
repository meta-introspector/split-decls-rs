// Generated macro for macro_19743 (macro)
macro_rules! Depcrate_um_adhocmacro_19743 {
() => {
// Module: crate::um::adhoc
// Provides: {"macro_19743"}
// Dependencies: {}
RIDL ! { # [uuid (0x8f10cc26 , 0xcf0d , 0x42a0 , 0xac , 0xbe , 0xe2 , 0xde , 0x70 , 0x07 , 0x38 , 0x4d)] interface IDot11AdHocManager (IDot11AdHocManagerVtbl) : IUnknown (IUnknownVtbl) { fn CreateNetwork (Name : LPCWSTR , Password : LPCWSTR , GeographicalId : LONG , pInterface : * mut IDot11AdHocInterface , pSecurity : * mut IDot11AdHocSecuritySettings , pContextGuid : * mut GUID , pIAdHoc : * mut * mut IDot11AdHocNetwork ,) -> HRESULT , fn CommitCreatedNetwork (pIAdHoc : * mut IDot11AdHocNetwork , fSaveProfile : BOOLEAN , fMakeSavedProfileUserSpecific : BOOLEAN ,) -> HRESULT , fn GetIEnumDot11AdHocNetworks (pContextGuid : * mut GUID , ppEnum : * mut * mut IEnumDot11AdHocNetworks ,) -> HRESULT , fn GetIEnumDot11AdHocInterfaces (ppEnum : * mut * mut IEnumDot11AdHocInterfaces ,) -> HRESULT , fn GetNetwork (NetworkSignature : * mut GUID , pNetwork : * mut * mut IDot11AdHocNetwork ,) -> HRESULT , } }
};
}
