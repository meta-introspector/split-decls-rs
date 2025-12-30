// Generated macro for macro_39776 (macro)
macro_rules! Depcrate_um_wbemprovmacro_39776 {
() => {
// Module: crate::um::wbemprov
// Provides: {"macro_39776"}
// Dependencies: {}
RIDL ! { # [uuid (0x1005cbcf , 0xe64f , 0x4646 , 0xbc , 0xd3 , 0x3a , 0x08 , 0x9d , 0x8a , 0x84 , 0xb4)] interface IWbemDecoupledRegistrar (IWbemDecoupledRegistrarVtbl) : IUnknown (IUnknownVtbl) { fn Register (a_Flags : c_long , a_Context : * mut IWbemContext , a_User : LPCWSTR , a_Locale : LPCWSTR , a_Scope : LPCWSTR , a_Registration : LPCWSTR , pIUnknown : * mut IUnknown ,) -> HRESULT , fn UnRegister () -> HRESULT , } }
};
}
