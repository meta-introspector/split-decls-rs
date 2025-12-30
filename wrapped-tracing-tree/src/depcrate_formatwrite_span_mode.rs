// Generated macro for write_span_mode (function)
macro_rules! Depcrate_formatwrite_span_mode {
() => {
// Module: crate::format
// Provides: {"write_span_mode"}
// Dependencies: {}
pub (crate) fn write_span_mode (buf : & mut String , style : SpanMode) { match style { SpanMode :: Open { verbose : true } => buf . push_str ("open(v)") , SpanMode :: Open { verbose : false } => buf . push_str ("open") , SpanMode :: Retrace { verbose : false } => buf . push_str ("retrace") , SpanMode :: Retrace { verbose : true } => buf . push_str ("retrace(v)") , SpanMode :: Close { verbose : true } => buf . push_str ("close(v)") , SpanMode :: Close { verbose : false } => buf . push_str ("close") , SpanMode :: PreOpen => buf . push_str ("pre_open") , SpanMode :: PostClose => buf . push_str ("post_close") , SpanMode :: Event => buf . push_str ("event") , } buf . push_str (": ") }
};
}
