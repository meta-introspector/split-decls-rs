// Generated macro for get_computer_name (function)
macro_rules! Depcrate_sysinfoget_computer_name {
() => {
// Module: crate::sysinfo
// Provides: {"get_computer_name"}
// Dependencies: {}
# [doc = " Retrieves a NetBIOS or DNS name associated with the local computer."] # [doc = ""] # [doc = " The names are established at system startup, when the system reads them"] # [doc = " from the registry."] # [doc = ""] # [doc = " This corresponds to calling [`GetComputerNameExW`]."] # [doc = ""] # [doc = " [`GetComputerNameExW`]: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw"] pub fn get_computer_name (kind : ComputerNameKind) -> io :: Result < OsString > { use std :: os :: windows :: ffi :: OsStringExt ; let format = kind . to_format () ; let mut len1 = 0 ; let _ = unsafe { GetComputerNameExW (format , std :: ptr :: null_mut () , & mut len1) } ; let len = match usize :: try_from (len1) { Ok (len) => len , Err (_) => { return Err (io :: Error :: new (io :: ErrorKind :: Other , "GetComputerNameExW buffer length overflowed usize" ,)) } } ; let mut buf = vec ! [0 ; len] ; let mut len2 = len1 ; let rc = unsafe { GetComputerNameExW (format , buf . as_mut_ptr () , & mut len2) } ; if rc == 0 { return Err (io :: Error :: last_os_error ()) ; } if len1 <= len2 { let msg = format ! ("GetComputerNameExW buffer length mismatch, \
             expected length strictly less than {} \
             but got {}" , len1 , len2 ,) ; return Err (io :: Error :: new (io :: ErrorKind :: Other , msg)) ; } let len = usize :: try_from (len2) . expect ("len1 fits implies len2 fits") ; Ok (OsString :: from_wide (& buf [.. len])) }
};
}
