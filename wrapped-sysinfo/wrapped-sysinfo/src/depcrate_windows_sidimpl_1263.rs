// Generated macro for impl_1263 (impl)
macro_rules! Depcrate_windows_sidimpl_1263 {
() => {
// Module: crate::windows::sid
// Provides: {"impl_1263"}
// Dependencies: {}
impl Sid { # [doc = " Creates an `Sid` by making a copy of the given raw SID."] pub (crate) unsafe fn from_psid (psid : PSID) -> Option < Self > { if psid . is_invalid () { return None ; } if unsafe { ! IsValidSid (psid) . as_bool () } { return None ; } let length = unsafe { GetLengthSid (psid) } ; let mut sid = vec ! [0 ; length as usize] ; if unsafe { CopySid (length , PSID (sid . as_mut_ptr () . cast ()) , psid) . is_err () } { sysinfo_debug ! ("CopySid failed: {:?}" , std :: io :: Error :: last_os_error ()) ; return None ; } assert_eq ! (sid [0] , 1 , "Expected SID revision to be 1") ; Some (Self { sid }) } # [doc = " Retrieves the account name of this SID."] # [cfg (feature = "user")] pub (crate) fn account_name (& self) -> Option < String > { unsafe { let mut name_len = 0 ; let mut domain_len = 0 ; let mut name_use = SidTypeUnknown ; let sid = PSID ((self . sid . as_ptr () as * mut u8) . cast ()) ; if let Err (err) = LookupAccountSidW (PCWSTR :: null () , sid , None , & mut name_len , None , & mut domain_len , & mut name_use ,) && err . code () != ERROR_INSUFFICIENT_BUFFER . to_hresult () { sysinfo_debug ! ("LookupAccountSidW failed: {:?}" , err) ; return None ; } let mut name = vec ! [0 ; name_len as usize] ; domain_len = 0 ; if LookupAccountSidW (PCWSTR :: null () , sid , Some (PWSTR :: from_raw (name . as_mut_ptr ())) , & mut name_len , None , & mut domain_len , & mut name_use ,) . is_err () { sysinfo_debug ! ("LookupAccountSidW failed: {:?}" , std :: io :: Error :: last_os_error ()) ; return None ; } Some (to_utf8_str (PWSTR :: from_raw (name . as_mut_ptr ()))) } } }
};
}
