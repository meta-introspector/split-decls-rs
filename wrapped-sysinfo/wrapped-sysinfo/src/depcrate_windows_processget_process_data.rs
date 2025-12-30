// Generated macro for get_process_data (function)
macro_rules! Depcrate_windows_processget_process_data {
() => {
// Module: crate::windows::process
// Provides: {"get_process_data"}
// Dependencies: {}
unsafe fn get_process_data (handle : HANDLE , ptr : * const c_void , size : usize ,) -> Result < Vec < u16 > , & 'static str > { let mut buffer : Vec < u16 > = Vec :: with_capacity (size / 2 + 1) ; let mut bytes_read = 0 ; unsafe { if ReadProcessMemory (handle , ptr , buffer . as_mut_ptr () . cast () , size , Some (& mut bytes_read) ,) . is_err () { return Err ("Unable to read process data") ; } if bytes_read != size { return Err ("ReadProcessMemory returned unexpected number of bytes read") ; } buffer . set_len (size / 2) ; buffer . push (0) ; } Ok (buffer) }
};
}
