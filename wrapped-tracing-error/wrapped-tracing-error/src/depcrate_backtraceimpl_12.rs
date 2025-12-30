// Generated macro for impl_12 (impl)
macro_rules! Depcrate_backtraceimpl_12 {
() => {
// Module: crate::backtrace
// Provides: {"impl_12"}
// Dependencies: {}
impl fmt :: Debug for SpanTrace { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct DebugSpan < 'a > { metadata : & 'a Metadata < 'a > , fields : & 'a str , } impl fmt :: Debug for DebugSpan < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{{ target: {:?}, name: {:?}" , self . metadata . target () , self . metadata . name ()) ? ; if ! self . fields . is_empty () { write ! (f , ", fields: {:?}" , self . fields) ? ; } if let Some ((file , line)) = self . metadata . file () . and_then (| file | self . metadata . line () . map (| line | (file , line))) { write ! (f , ", file: {:?}, line: {:?}" , file , line) ? ; } write ! (f , " }}") ? ; Ok (()) } } write ! (f , "SpanTrace ") ? ; let mut dbg = f . debug_list () ; self . with_spans (| metadata , fields | { dbg . entry (& DebugSpan { metadata , fields }) ; true }) ; dbg . finish () } }
};
}
