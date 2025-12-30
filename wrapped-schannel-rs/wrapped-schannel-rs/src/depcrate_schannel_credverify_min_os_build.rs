// Generated macro for verify_min_os_build (function)
macro_rules! Depcrate_schannel_credverify_min_os_build {
() => {
// Module: crate::schannel_cred
// Provides: {"verify_min_os_build"}
// Dependencies: {}
fn verify_min_os_build (major : u32 , build : u32) -> Option < () > { use windows_sys :: Win32 :: System :: SystemInformation :: OSVERSIONINFOW ; let handle = std :: ptr :: NonNull :: new (unsafe { windows_sys :: Win32 :: System :: LibraryLoader :: GetModuleHandleW (windows_sys :: w ! ("ntdll.dll")) }) ? ; let rtl_get_ver = unsafe { windows_sys :: Win32 :: System :: LibraryLoader :: GetProcAddress (handle . as_ptr () , windows_sys :: s ! ("RtlGetVersion")) } ? ; type RtlGetVersionFunc = unsafe extern "system" fn (* mut OSVERSIONINFOW) -> i32 ; let proc : RtlGetVersionFunc = unsafe { mem :: transmute (rtl_get_ver) } ; let mut info : OSVERSIONINFOW = unsafe { mem :: zeroed () } ; info . dwOSVersionInfoSize = mem :: size_of :: < OSVERSIONINFOW > () as u32 ; unsafe { proc (& mut info) } ; if info . dwMajorVersion > major || (info . dwMajorVersion == major && info . dwBuildNumber >= build) { Some (()) } else { None } }
};
}
