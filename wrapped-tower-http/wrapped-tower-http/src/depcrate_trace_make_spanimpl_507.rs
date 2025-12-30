// Generated macro for impl_507 (impl)
macro_rules! Depcrate_trace_make_spanimpl_507 {
() => {
// Module: crate::trace::make_span
// Provides: {"impl_507"}
// Dependencies: {}
impl < B > MakeSpan < B > for DefaultMakeSpan { fn make_span (& mut self , request : & Request < B >) -> Span { macro_rules ! make_span { ($ level : expr) => { if self . include_headers { tracing :: span ! ($ level , "request" , method = % request . method () , uri = % request . uri () , version = ? request . version () , headers = ? request . headers () ,) } else { tracing :: span ! ($ level , "request" , method = % request . method () , uri = % request . uri () , version = ? request . version () ,) } } } match self . level { Level :: ERROR => make_span ! (Level :: ERROR) , Level :: WARN => make_span ! (Level :: WARN) , Level :: INFO => make_span ! (Level :: INFO) , Level :: DEBUG => make_span ! (Level :: DEBUG) , Level :: TRACE => make_span ! (Level :: TRACE) , } } }
};
}
