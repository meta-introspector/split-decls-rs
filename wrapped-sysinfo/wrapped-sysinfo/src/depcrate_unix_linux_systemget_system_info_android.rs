// Generated macro for get_system_info_android (function)
macro_rules! Depcrate_unix_linux_systemget_system_info_android {
() => {
// Module: crate::unix::linux::system
// Provides: {"get_system_info_android"}
// Dependencies: {}
# [cfg (target_os = "android")] fn get_system_info_android (info : InfoType) -> Option < String > { let name : & 'static [u8] = match info { InfoType :: Name => b"ro.product.model\0" , InfoType :: OsVersion => b"ro.build.version.release\0" , InfoType :: DistributionID => { return None ; } InfoType :: DistributionIDLike => { return None ; } } ; let mut value_buffer = vec ! [0u8 ; libc :: PROP_VALUE_MAX as usize] ; unsafe { let len = libc :: __system_property_get (name . as_ptr () as * const c_char , value_buffer . as_mut_ptr () as * mut c_char ,) ; if len != 0 { if let Some (pos) = value_buffer . iter () . position (| c | * c == 0) { value_buffer . resize (pos , 0) ; } String :: from_utf8 (value_buffer) . ok () } else { None } } }
};
}
