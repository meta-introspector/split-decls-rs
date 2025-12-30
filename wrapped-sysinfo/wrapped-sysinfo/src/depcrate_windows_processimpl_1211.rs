// Generated macro for impl_1211 (impl)
macro_rules! Depcrate_windows_processimpl_1211 {
() => {
// Module: crate::windows::process
// Provides: {"impl_1211"}
// Dependencies: {}
impl fmt :: Display for ProcessStatus { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match * self { ProcessStatus :: Run => "Runnable" , _ => "Unknown" , }) } }
};
}
