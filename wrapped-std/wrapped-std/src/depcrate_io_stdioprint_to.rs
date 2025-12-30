// Generated macro for print_to (function)
macro_rules! Depcrate_io_stdioprint_to {
() => {
// Module: crate::io::stdio
// Provides: {"print_to"}
// Dependencies: {}
# [doc = " Writes `args` to the capture buffer if enabled and possible, or `global_s`"] # [doc = " otherwise. `label` identifies the stream in a panic message."] # [doc = ""] # [doc = " This function is used to print error messages, so it takes extra"] # [doc = " care to avoid causing a panic when `OUTPUT_CAPTURE` is unusable."] # [doc = " For instance, if the TLS key for output capturing is already destroyed, or"] # [doc = " if the local stream is in use by another thread, it will just fall back to"] # [doc = " the global stream."] # [doc = ""] # [doc = " However, if the actual I/O causes an error, this function does panic."] # [doc = ""] # [doc = " Writing to non-blocking stdout/stderr can cause an error, which will lead"] # [doc = " this function to panic."] fn print_to < T > (args : fmt :: Arguments < '_ > , global_s : fn () -> T , label : & str) where T : Write , { if print_to_buffer_if_capture_used (args) { return ; } if let Err (e) = global_s () . write_fmt (args) { panic ! ("failed printing to {label}: {e}") ; } }
};
}
