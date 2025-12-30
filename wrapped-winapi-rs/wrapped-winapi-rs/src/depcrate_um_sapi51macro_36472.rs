// Generated macro for macro_36472 (macro)
macro_rules! Depcrate_um_sapi51macro_36472 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36472"}
// Dependencies: {}
RIDL ! { # [uuid (0x8445c581 , 0x0cac , 0x4a38 , 0xab , 0xfe , 0x9b , 0x2c , 0xe2 , 0x82 , 0x64 , 0x55)] interface ISpPhoneConverter (ISpPhoneConverterVtbl) : ISpObjectWithToken (ISpObjectWithTokenVtbl) { fn PhoneToId (pszPhone : LPCWSTR , pId : * mut SPPHONEID ,) -> HRESULT , fn IdToPhone (pId : PCSPPHONEID , pszPhone : * mut WCHAR ,) -> HRESULT , } }
};
}
