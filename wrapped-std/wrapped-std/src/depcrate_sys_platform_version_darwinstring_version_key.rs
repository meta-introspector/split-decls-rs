// Generated macro for string_version_key (function)
macro_rules! Depcrate_sys_platform_version_darwinstring_version_key {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"string_version_key"}
// Dependencies: {}
# [doc = " Look up a string key in a CFDictionary, and convert it to an [`OSVersion`]."] unsafe fn string_version_key (cf_handle : & CFHandle , plist : CFDictionaryRef , lookup_key : & CStr ,) -> Option < OSVersion > { let cf_lookup_key = unsafe { cf_handle . CFStringCreateWithCStringNoCopy (kCFAllocatorDefault , lookup_key . as_ptr () , kCFStringEncodingUTF8 , cf_handle . kCFAllocatorNull () ,) } ; assert ! (! cf_lookup_key . is_null () , "failed creating CFString") ; let _lookup_key_release = Deferred (| | unsafe { cf_handle . CFRelease (cf_lookup_key) }) ; let value : CFTypeRef = unsafe { cf_handle . CFDictionaryGetValue (plist , cf_lookup_key) } . cast_mut () ; if value . is_null () { return None ; } assert_eq ! (unsafe { cf_handle . CFGetTypeID (value) } , unsafe { cf_handle . CFStringGetTypeID () } , "key in SystemVersion.plist must be a string") ; let value : CFStringRef = value . cast () ; let mut version_str = [0u8 ; 32] ; let ret = unsafe { cf_handle . CFStringGetCString (value , version_str . as_mut_ptr () . cast :: < c_char > () , version_str . len () as CFIndex , kCFStringEncodingUTF8 ,) } ; assert_ne ! (ret , 0 , "failed getting string from CFString") ; let version_str = CStr :: from_bytes_until_nul (& version_str) . expect ("failed converting CFString to CStr") ; Some (parse_os_version (version_str . to_bytes ()) . unwrap_or_else (| err | { panic ! ("failed parsing version from PList ({}): {err}" , ByteStr :: new (version_str . to_bytes ())) })) }
};
}
