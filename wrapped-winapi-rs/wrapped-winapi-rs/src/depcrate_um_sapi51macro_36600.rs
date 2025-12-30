// Generated macro for macro_36600 (macro)
macro_rules! Depcrate_um_sapi51macro_36600 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36600"}
// Dependencies: {}
RIDL ! { # [uuid (0xce17c09b , 0x4efa , 0x44d5 , 0xa4 , 0xc9 , 0x59 , 0xd9 , 0x58 , 0x5a , 0xb0 , 0xcd)] interface ISpeechDataKey (ISpeechDataKeyVtbl) : IDispatch (IDispatchVtbl) { fn SetBinaryValue (ValueName : BSTR , Value : VARIANT ,) -> HRESULT , fn GetBinaryValue (ValueName : BSTR , Value : * mut VARIANT ,) -> HRESULT , fn SetStringValue (ValueName : BSTR , Value : BSTR ,) -> HRESULT , fn GetStringValue (ValueName : BSTR , Value : * mut BSTR ,) -> HRESULT , fn SetLongValue (ValueName : BSTR , Value : c_long ,) -> HRESULT , fn GetLongValue (ValueName : BSTR , Value : * mut c_long ,) -> HRESULT , fn OpenKey (SubKeyName : BSTR , SubKey : * mut * mut ISpeechDataKey ,) -> HRESULT , fn CreateKey (SubKeyName : BSTR , SubKey : * mut * mut ISpeechDataKey ,) -> HRESULT , fn DeleteKey (SubKeyName : BSTR ,) -> HRESULT , fn DeleteValue (ValueName : BSTR ,) -> HRESULT , fn EnumKeys (Index : c_long , SubKeyName : * mut BSTR ,) -> HRESULT , fn EnumValues (Index : c_long , ValueName : * mut BSTR ,) -> HRESULT , } }
};
}
