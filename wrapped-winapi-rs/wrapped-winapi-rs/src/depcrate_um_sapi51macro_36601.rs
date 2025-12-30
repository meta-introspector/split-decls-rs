// Generated macro for macro_36601 (macro)
macro_rules! Depcrate_um_sapi51macro_36601 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36601"}
// Dependencies: {}
RIDL ! { # [uuid (0xc74a3adc , 0xb727 , 0x4500 , 0xa8 , 0x4a , 0xb5 , 0x26 , 0x72 , 0x1c , 0x8b , 0x8c)] interface ISpeechObjectToken (ISpeechObjectTokenVtbl) : IDispatch (IDispatchVtbl) { fn get_Id (ObjectId : * mut BSTR ,) -> HRESULT , fn get_DataKey (DataKey : * mut * mut ISpeechDataKey ,) -> HRESULT , fn get_Category (Category : * mut * mut ISpeechObjectTokenCategory ,) -> HRESULT , fn GetDescription (Locale : c_long , Description : * mut BSTR ,) -> HRESULT , fn SetId (Id : BSTR , CategoryId : BSTR , CreateIfNotExist : VARIANT_BOOL ,) -> HRESULT , fn GetAttribute (AttributeName : BSTR , AttributeValue : * mut BSTR ,) -> HRESULT , fn CreateInstance (pUnkOuter : * mut IUnknown , ClsContext : SpeechTokenContext , Object : * mut * mut IUnknown ,) -> HRESULT , fn Remove (ObjectStorageCLSID : BSTR ,) -> HRESULT , fn GetStorageFileName (ObjectStorageCLSID : BSTR , KeyName : BSTR , FileName : BSTR , Folder : BSTR , FilePath : * mut BSTR ,) -> HRESULT , fn RemoveStorageFileName (ObjectStorageCLSID : BSTR , KeyName : BSTR , DeleteFile : VARIANT_BOOL ,) -> HRESULT , fn IsUISupported (TypeOfUI : BSTR , ExtraData : * const VARIANT , Object : * mut IUnknown , Supported : * mut VARIANT_BOOL ,) -> HRESULT , fn DisplayUI (hWnd : c_long , Title : BSTR , TypeOfUI : BSTR , ExtraData : * const VARIANT , Object : * mut IUnknown ,) -> HRESULT , fn MatchesAttributes (Attributes : BSTR , Matches : * mut VARIANT_BOOL ,) -> HRESULT , } }
};
}
