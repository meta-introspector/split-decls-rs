// Generated macro for impl_3535 (impl)
macro_rules! Depcrate_sys_backtraceimpl_3535 {
() => {
// Module: crate::sys::backtrace
// Provides: {"impl_3535"}
// Dependencies: {}
impl BacktraceLock < '_ > { # [doc = " Prints the current backtrace."] # [doc = ""] # [doc = " NOTE: this function is not Sync. The caller must hold a mutex lock, or there must be only one thread in the program."] pub (crate) fn print (& mut self , w : & mut dyn Write , format : PrintFmt) -> io :: Result < () > { if cfg ! (test) { return Ok (()) ; } struct DisplayBacktrace { format : PrintFmt , } impl fmt :: Display for DisplayBacktrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { unsafe { _print_fmt (fmt , self . format) } } } write ! (w , "{}" , DisplayBacktrace { format }) } }
};
}
