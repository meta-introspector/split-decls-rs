// Generated macro for macro_36603 (macro)
macro_rules! Depcrate_um_sapi51macro_36603 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36603"}
// Dependencies: {}
RIDL ! { # [uuid (0xca7eac50 , 0x2d01 , 0x4145 , 0x86 , 0xd4 , 0x5a , 0xe7 , 0xd7 , 0x0f , 0x44 , 0x69)] interface ISpeechObjectTokenCategory (ISpeechObjectTokenCategoryVtbl) : IDispatch (IDispatchVtbl) { fn get_Id (Id : * mut BSTR ,) -> HRESULT , fn put_Default (TokenId : BSTR ,) -> HRESULT , fn get_Default (TokenId : * mut BSTR ,) -> HRESULT , fn SetId (Id : BSTR , CreateIfNotExist : VARIANT_BOOL ,) -> HRESULT , fn GetDataKey (Location : SpeechDataKeyLocation , DataKey : * mut * mut ISpeechDataKey ,) -> HRESULT , fn EnumerateTokens (RequiredAttributes : BSTR , OptionalAttributes : BSTR , Tokens : * mut * mut ISpeechObjectTokens ,) -> HRESULT , } }
};
}
