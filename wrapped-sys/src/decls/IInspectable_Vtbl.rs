macro_rules! deps {
    () => {
        GUID!();
        HRESULT!();
        IUnknown_Vtbl!();
    };
}

macro_rules! IInspectable_Vtbl {
    () => {
        deps!();
        # [repr (C)] pub struct IInspectable_Vtbl { pub base : IUnknown_Vtbl , pub GetIids : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , count : * mut u32 , values : * mut * mut GUID) -> HRESULT , pub GetRuntimeClassName : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , value : * mut * mut core :: ffi :: c_void) -> HRESULT , pub GetTrustLevel : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , value : * mut i32) -> HRESULT , }
    };
}

IInspectable_Vtbl!()