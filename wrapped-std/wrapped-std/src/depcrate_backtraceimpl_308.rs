// Generated macro for impl_308 (impl)
macro_rules! Depcrate_backtraceimpl_308 {
() => {
// Module: crate::backtrace
// Provides: {"impl_308"}
// Dependencies: {}
impl fmt :: Debug for BytesOrWide { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { output_filename (fmt , match self { BytesOrWide :: Bytes (w) => BytesOrWideString :: Bytes (w) , BytesOrWide :: Wide (w) => BytesOrWideString :: Wide (w) , } , backtrace_rs :: PrintFmt :: Short , crate :: env :: current_dir () . as_ref () . ok () ,) } }
};
}
