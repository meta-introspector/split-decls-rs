// Generated macro for get_system_info (function)
macro_rules! Depcrate_unix_apple_systemget_system_info {
() => {
// Module: crate::unix::apple::system
// Provides: {"get_system_info"}
// Dependencies: {}
fn get_system_info (value : c_int , default : Option < & str >) -> Option < String > { let mut mib : [c_int ; 2] = [libc :: CTL_KERN , value] ; let mut size = 0 ; unsafe { sysctl (mib . as_mut_ptr () , mib . len () as _ , std :: ptr :: null_mut () , & mut size , std :: ptr :: null_mut () , 0 ,) ; if size == 0 { default . map (| s | s . to_owned ()) } else { let mut buf = vec ! [0_u8 ; size as _] ; if sysctl (mib . as_mut_ptr () , mib . len () as _ , buf . as_mut_ptr () as _ , & mut size , std :: ptr :: null_mut () , 0 ,) == - 1 { default . map (| s | s . to_owned ()) } else { if let Some (pos) = buf . iter () . position (| x | * x == 0) { buf . resize (pos , 0) ; } String :: from_utf8 (buf) . ok () } } } }
};
}
