// Generated macro for get_system_info (function)
macro_rules! Depcrate_unix_freebsd_systemget_system_info {
() => {
// Module: crate::unix::freebsd::system
// Provides: {"get_system_info"}
// Dependencies: {}
fn get_system_info (mib : & [c_int] , default : Option < & str >) -> Option < String > { let mut size = 0 ; unsafe { libc :: sysctl (mib . as_ptr () , mib . len () as _ , std :: ptr :: null_mut () , & mut size , std :: ptr :: null_mut () , 0 ,) ; if size == 0 { default . map (| s | s . to_owned ()) } else { let mut buf : Vec < libc :: c_char > = vec ! [0 ; size as _] ; if libc :: sysctl (mib . as_ptr () , mib . len () as _ , buf . as_mut_ptr () as _ , & mut size , std :: ptr :: null_mut () , 0 ,) == - 1 { default . map (| s | s . to_owned ()) } else { c_buf_to_utf8_string (& buf) } } } }
};
}
