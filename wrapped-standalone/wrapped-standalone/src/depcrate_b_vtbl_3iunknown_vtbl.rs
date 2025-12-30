// Generated macro for IUnknown_Vtbl (struct)
macro_rules! Depcrate_b_vtbl_3IUnknown_Vtbl {
() => {
// Module: crate::b_vtbl_3
// Provides: {"IUnknown_Vtbl"}
// Dependencies: {}
# [repr (C)] pub struct IUnknown_Vtbl { pub QueryInterface : unsafe extern "system" fn (this : * mut core :: ffi :: c_void , iid : * const GUID , interface : * mut * mut core :: ffi :: c_void ,) -> HRESULT , pub AddRef : unsafe extern "system" fn (this : * mut core :: ffi :: c_void) -> u32 , pub Release : unsafe extern "system" fn (this : * mut core :: ffi :: c_void) -> u32 , }
};
}
