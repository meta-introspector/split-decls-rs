// Generated macro for ph_query_process_variable_size (function)
macro_rules! Depcrate_windows_processph_query_process_variable_size {
() => {
// Module: crate::windows::process
// Provides: {"ph_query_process_variable_size"}
// Dependencies: {}
unsafe fn ph_query_process_variable_size (process_handle : HANDLE , process_information_class : PROCESSINFOCLASS ,) -> Option < Vec < u16 > > { let mut return_length = MaybeUninit :: < u32 > :: uninit () ; unsafe { if let Err (err) = NtQueryInformationProcess (process_handle , process_information_class as _ , null_mut () , 0 , return_length . as_mut_ptr () as * mut _ ,) . ok () && ! [STATUS_BUFFER_OVERFLOW . into () , STATUS_BUFFER_TOO_SMALL . into () , STATUS_INFO_LENGTH_MISMATCH . into () ,] . contains (& err . code ()) { return None ; } let mut return_length = return_length . assume_init () ; let buf_len = (return_length as usize) / 2 ; let mut buffer : Vec < u16 > = Vec :: with_capacity (buf_len + 1) ; if NtQueryInformationProcess (process_handle , process_information_class as _ , buffer . as_mut_ptr () as * mut _ , return_length , & mut return_length as * mut _ ,) . is_err () { return None ; } buffer . set_len (buf_len) ; buffer . push (0) ; Some (buffer) } }
};
}
