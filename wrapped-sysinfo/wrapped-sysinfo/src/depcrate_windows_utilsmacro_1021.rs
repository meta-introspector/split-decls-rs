// Generated macro for macro_1021 (macro)
macro_rules! Depcrate_windows_utilsmacro_1021 {
() => {
// Module: crate::windows::utils
// Provides: {"macro_1021"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (feature = "disk" , feature = "system"))] { use windows :: Win32 :: Foundation :: { CloseHandle , HANDLE } ; use std :: ops :: Deref ; pub (crate) struct HandleWrapper (pub (crate) HANDLE) ; impl HandleWrapper { # [cfg (feature = "system")] pub (crate) fn new (handle : HANDLE) -> Option < Self > { if handle . is_invalid () { None } else { Some (Self (handle)) } } # [cfg (feature = "disk")] pub (crate) unsafe fn new_from_file (drive_name : & [u16] , open_rights : FILE_ACCESS_RIGHTS ,) -> Option < Self > { let lpfilename = windows :: core :: PCWSTR :: from_raw (drive_name . as_ptr ()) ; let handle = unsafe { CreateFileW (lpfilename , open_rights . 0 , FILE_SHARE_READ | FILE_SHARE_WRITE , None , OPEN_EXISTING , Default :: default () , Some (HANDLE :: default ()) ,) } . ok () ?; if handle . is_invalid () { sysinfo_debug ! ("Expected handle to {:?} to be valid" , String :: from_utf16_lossy (drive_name)) ; None } else { Some (Self (handle)) } } } impl Deref for HandleWrapper { type Target = HANDLE ; fn deref (& self) -> & Self :: Target { & self . 0 } } impl Drop for HandleWrapper { fn drop (& mut self) { let _err = unsafe { CloseHandle (self . 0) } ; } } } }
};
}
