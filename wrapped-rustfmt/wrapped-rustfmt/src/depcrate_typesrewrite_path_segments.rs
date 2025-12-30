// Generated macro for rewrite_path_segments (function)
macro_rules! Depcrate_typesrewrite_path_segments {
() => {
// Module: crate::types
// Provides: {"rewrite_path_segments"}
// Dependencies: {}
fn rewrite_path_segments < 'a , I > (path_context : PathContext , mut buffer : String , iter : I , mut span_lo : BytePos , span_hi : BytePos , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult where I : Iterator < Item = & 'a ast :: PathSegment > , { let mut first = true ; let shape = shape . visual_indent (0) ; for segment in iter { if segment . ident . name == kw :: PathRoot { continue ; } if first { first = false ; } else { buffer . push_str ("::") ; } let extra_offset = extra_offset (& buffer , shape) ; let new_shape = shape . shrink_left (extra_offset , mk_sp (span_lo , span_hi)) ? ; let segment_string = rewrite_segment (path_context , segment , & mut span_lo , span_hi , context , new_shape ,) ? ; buffer . push_str (& segment_string) ; } Ok (buffer) }
};
}
