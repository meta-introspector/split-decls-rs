// Generated macro for impl_140 (impl)
macro_rules! Depcrate_low_level_siginfoimpl_140 {
() => {
// Module: crate::low_level::siginfo
// Provides: {"impl_140"}
// Dependencies: {}
impl Debug for Origin { fn fmt (& self , fmt : & mut Formatter) -> FmtResult { fn named_signal (sig : c_int) -> String { low_level :: signal_name (sig) . map (| n | format ! ("{} ({})" , n , sig)) . unwrap_or_else (| | sig . to_string ()) } fmt . debug_struct ("Origin") . field ("signal" , & named_signal (self . signal)) . field ("process" , & self . process) . field ("cause" , & self . cause) . finish () } }
};
}
