// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [export_name = "efi_main"] pub extern "C" fn main (_h : Handle , st : * mut SystemTable) -> Status { let s = b"Hello World!\n\0" . map (| c | u16 :: from (c)) ; let r = unsafe { ((* (* st) . con_out) . output_string) ((* st) . con_out , s . as_ptr () as * mut Char16) } ; if r . is_error () { return r ; } unsafe { ((* ((* st) . runtime_services)) . reset_system) (RESET_SHUTDOWN , Status :: SUCCESS , 0 , ptr :: null_mut () ,) ; } Status :: UNSUPPORTED }
};
}
