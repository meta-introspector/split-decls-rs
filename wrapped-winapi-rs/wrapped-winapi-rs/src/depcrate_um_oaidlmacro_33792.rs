// Generated macro for macro_33792 (macro)
macro_rules! Depcrate_um_oaidlmacro_33792 {
() => {
// Module: crate::um::oaidl
// Provides: {"macro_33792"}
// Dependencies: {}
RIDL ! { # [uuid (0x0000002F , 0x0000 , 0x0000 , 0xC0 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x46)] interface IRecordInfo (IRecordInfoVtbl) : IUnknown (IUnknownVtbl) { fn RecordInit (pvNew : PVOID ,) -> HRESULT , fn RecordClear (pvExisting : PVOID ,) -> HRESULT , fn RecordCopy (pvExisting : PVOID , pvNew : PVOID ,) -> HRESULT , fn GetGuid (pguid : * mut GUID ,) -> HRESULT , fn GetName (pbstrName : * mut BSTR ,) -> HRESULT , fn GetSize (pcbSize : * mut ULONG ,) -> HRESULT , fn GetTypeInfo (ppTypeInfo : * mut * mut ITypeInfo ,) -> HRESULT , fn GetField (pvData : PVOID , szFieldName : LPCOLESTR , pvarField : * mut VARIANT ,) -> HRESULT , fn GetFieldNoCopy (pvData : PVOID , szFieldName : LPCOLESTR , pvarField : * mut VARIANT , ppvDataCArray : * mut PVOID ,) -> HRESULT , fn PutField (wFlags : ULONG , pvData : PVOID , szFieldName : LPCOLESTR , pvarField : * mut VARIANT ,) -> HRESULT , fn PutFieldNoCopy (wFlags : ULONG , pvData : PVOID , szFieldName : LPCOLESTR , pvarField : * mut VARIANT ,) -> HRESULT , fn GetFieldNames (pcNames : * mut ULONG , rgBstrNames : * mut BSTR ,) -> HRESULT , fn IsMatchingType (pRecordInfo : * mut IRecordInfo ,) -> BOOL , fn RecordCreate () -> PVOID , fn RecordCreateCopy (pvSource : PVOID , ppvDest : * mut PVOID ,) -> HRESULT , fn RecordDestroy (pvRecord : PVOID ,) -> HRESULT , } }
};
}
