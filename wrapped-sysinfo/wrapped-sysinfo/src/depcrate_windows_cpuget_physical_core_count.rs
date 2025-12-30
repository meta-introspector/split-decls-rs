// Generated macro for get_physical_core_count (function)
macro_rules! Depcrate_windows_cpuget_physical_core_count {
() => {
// Module: crate::windows::cpu
// Provides: {"get_physical_core_count"}
// Dependencies: {}
pub (crate) fn get_physical_core_count () -> Option < usize > { let mut needed_size = 0 ; unsafe { let _err = GetLogicalProcessorInformationEx (RelationAll , None , & mut needed_size) ; let mut buf : Vec < u8 > = Vec :: with_capacity (needed_size as _) ; loop { buf . set_len (needed_size as _) ; if GetLogicalProcessorInformationEx (RelationAll , Some (buf . as_mut_ptr () . cast ()) , & mut needed_size ,) . is_ok () { break ; } else { let e = Error :: last_os_error () ; match e . raw_os_error () { Some (value) if value == ERROR_INSUFFICIENT_BUFFER . 0 as i32 => { } _ => { sysinfo_debug ! ("get_physical_core_count: GetLogicalCpuInformationEx failed") ; return None ; } } } let reserve = if needed_size as usize > buf . capacity () { needed_size as usize - buf . capacity () } else { 1 } ; needed_size = match needed_size . checked_add (reserve as _) { Some (new_size) => new_size , None => { sysinfo_debug ! ("get_physical_core_count: buffer size is too big ({} + {})" , needed_size , reserve ,) ; return None ; } } ; buf . reserve (reserve) ; } buf . set_len (needed_size as _) ; let mut i = 0 ; let raw_buf = buf . as_ptr () ; let mut count = 0 ; while i < buf . len () { let p = & * (raw_buf . add (i) as * const SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX) ; i += p . Size as usize ; if p . Relationship == RelationProcessorCore { count += 1 ; } } Some (count) } }
};
}
