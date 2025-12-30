// Generated macro for impl_2991 (impl)
macro_rules! Depcrate_processimpl_2991 {
() => {
// Module: crate::process
// Provides: {"impl_2991"}
// Dependencies: {}
# [stable (feature = "process_output_debug" , since = "1.7.0")] impl fmt :: Debug for Output { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let stdout_utf8 = str :: from_utf8 (& self . stdout) ; let stdout_debug : & dyn fmt :: Debug = match stdout_utf8 { Ok (ref s) => s , Err (_) => & self . stdout , } ; let stderr_utf8 = str :: from_utf8 (& self . stderr) ; let stderr_debug : & dyn fmt :: Debug = match stderr_utf8 { Ok (ref s) => s , Err (_) => & self . stderr , } ; fmt . debug_struct ("Output") . field ("status" , & self . status) . field ("stdout" , stdout_debug) . field ("stderr" , stderr_debug) . finish () } }
};
}
