// Generated macro for parse_version_from_plist (function)
macro_rules! Depcrate_sys_platform_version_darwinparse_version_from_plist {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"parse_version_from_plist"}
// Dependencies: {}
# [doc = " Parse OS version from the given PList."] # [doc = ""] # [doc = " Split out from [`version_from_plist`] to allow for testing."] fn parse_version_from_plist (cf_handle : & CFHandle , plist_buffer : & [u8]) -> OSVersion { let plist_data = unsafe { cf_handle . CFDataCreateWithBytesNoCopy (kCFAllocatorDefault , plist_buffer . as_ptr () , plist_buffer . len () as CFIndex , cf_handle . kCFAllocatorNull () ,) } ; assert ! (! plist_data . is_null () , "failed creating CFData") ; let _plist_data_release = Deferred (| | unsafe { cf_handle . CFRelease (plist_data) }) ; let plist = unsafe { cf_handle . CFPropertyListCreateWithData (kCFAllocatorDefault , plist_data , kCFPropertyListImmutable , null_mut () , null_mut () ,) } ; assert ! (! plist . is_null () , "failed reading PList in SystemVersion.plist") ; let _plist_release = Deferred (| | unsafe { cf_handle . CFRelease (plist) }) ; assert_eq ! (unsafe { cf_handle . CFGetTypeID (plist) } , unsafe { cf_handle . CFDictionaryGetTypeID () } , "SystemVersion.plist did not contain a dictionary at the top level") ; let plist : CFDictionaryRef = plist . cast () ; if cfg ! (target_os = "ios") { if let Some (ios_support_version) = unsafe { string_version_key (cf_handle , plist , c"iOSSupportVersion") } { return ios_support_version ; } if cfg ! (target_abi = "macabi") { panic ! ("expected iOSSupportVersion in SystemVersion.plist") ; } } unsafe { string_version_key (cf_handle , plist , c"ProductVersion") } . expect ("expected ProductVersion in SystemVersion.plist") }
};
}
