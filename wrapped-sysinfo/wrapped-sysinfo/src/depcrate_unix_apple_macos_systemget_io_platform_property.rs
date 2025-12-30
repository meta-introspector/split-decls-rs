// Generated macro for get_io_platform_property (function)
macro_rules! Depcrate_unix_apple_macos_systemget_io_platform_property {
() => {
// Module: crate::unix::apple::macos::system
// Provides: {"get_io_platform_property"}
// Dependencies: {}
# [cfg (not (feature = "apple-sandbox"))] pub (crate) fn get_io_platform_property (key : & str) -> Option < String > { use crate :: sys :: macos :: utils :: IOReleaser ; use objc2_core_foundation :: { CFData , CFGetTypeID , CFString , ConcreteType , kCFAllocatorDefault } ; use objc2_io_kit :: { IORegistryEntryCreateCFProperty , IOServiceGetMatchingService , IOServiceMatching , kIOMainPortDefault , } ; use std :: ffi :: CStr ; let matching = match unsafe { IOServiceMatching (c"IOPlatformExpertDevice" . as_ptr () . cast ()) } { Some (matching) => matching , None => { sysinfo_debug ! ("IOServiceMatching call failed, `IOPlatformExpertDevice` not found") ; return None ; } } ; let result = unsafe { IOServiceGetMatchingService (kIOMainPortDefault , Some (matching . as_opaque () . into ())) } ; if result == 0 { sysinfo_debug ! ("IOServiceGetMatchingService failed") ; return None ; } let _r = IOReleaser :: new (result) ; let key_cfstring = CFString :: from_str (key) ; let properties = unsafe { IORegistryEntryCreateCFProperty (result , Some (& key_cfstring) , kCFAllocatorDefault , 0) } ? ; if CFGetTypeID (Some (& * properties)) == CFString :: type_id () { properties . downcast :: < CFString > () . ok () . map (| s | s . to_string ()) } else { properties . downcast :: < CFData > () . ok () . and_then (| s | { CStr :: from_bytes_with_nul (& s . to_vec ()) . ok () . and_then (| cstr | cstr . to_str () . ok ()) . map (str :: to_owned) }) } }
};
}
