// Generated macro for impl_11 (impl)
macro_rules! Depcrate_backtraceimpl_11 {
() => {
// Module: crate::backtrace
// Provides: {"impl_11"}
// Dependencies: {}
impl fmt :: Display for SpanTrace { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut err = Ok (()) ; let mut span = 0 ; self . with_spans (| metadata , fields | { if span > 0 { try_bool ! (write ! (f , "\n" ,) , err) ; } try_bool ! (write ! (f , "{:>4}: {}::{}" , span , metadata . target () , metadata . name ()) , err) ; if ! fields . is_empty () { try_bool ! (write ! (f , "\n           with {}" , fields) , err) ; } if let Some ((file , line)) = metadata . file () . and_then (| file | metadata . line () . map (| line | (file , line))) { try_bool ! (write ! (f , "\n             at {}:{}" , file , line) , err) ; } span += 1 ; true }) ; err } }
};
}
