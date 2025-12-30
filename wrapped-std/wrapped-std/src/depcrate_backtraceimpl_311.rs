// Generated macro for impl_311 (impl)
macro_rules! Depcrate_backtraceimpl_311 {
() => {
// Module: crate::backtrace
// Provides: {"impl_311"}
// Dependencies: {}
# [stable (feature = "backtrace" , since = "1.65.0")] impl fmt :: Display for Backtrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let capture = match & self . inner { Inner :: Unsupported => return fmt . write_str ("unsupported backtrace") , Inner :: Disabled => return fmt . write_str ("disabled backtrace") , Inner :: Captured (c) => & * * c , } ; let full = fmt . alternate () ; let (frames , style) = if full { (& capture . frames [..] , backtrace_rs :: PrintFmt :: Full) } else { (& capture . frames [capture . actual_start ..] , backtrace_rs :: PrintFmt :: Short) } ; let cwd = crate :: env :: current_dir () ; let mut print_path = move | fmt : & mut fmt :: Formatter < '_ > , path : BytesOrWideString < '_ > | { output_filename (fmt , path , style , cwd . as_ref () . ok ()) } ; let mut f = backtrace_rs :: BacktraceFmt :: new (fmt , style , & mut print_path) ; f . add_context () ? ; for frame in frames { if frame . symbols . is_empty () { f . frame () . print_raw (frame . frame . ip () , None , None , None) ? ; } else { for symbol in frame . symbols . iter () { f . frame () . print_raw_with_column (frame . frame . ip () , symbol . name . as_ref () . map (| b | backtrace_rs :: SymbolName :: new (b)) , symbol . filename . as_ref () . map (| b | match b { BytesOrWide :: Bytes (w) => BytesOrWideString :: Bytes (w) , BytesOrWide :: Wide (w) => BytesOrWideString :: Wide (w) , }) , symbol . lineno , symbol . colno ,) ? ; } } } f . finish () ? ; Ok (()) } }
};
}
