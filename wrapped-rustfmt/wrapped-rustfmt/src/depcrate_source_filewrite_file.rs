// Generated macro for write_file (function)
macro_rules! Depcrate_source_filewrite_file {
() => {
// Module: crate::source_file
// Provides: {"write_file"}
// Dependencies: {}
pub (crate) fn write_file < T > (psess : Option < & ParseSess > , filename : & FileName , formatted_text : & str , out : & mut T , emitter : & mut dyn Emitter , newline_style : NewlineStyle ,) -> Result < emitter :: EmitterResult , io :: Error > where T : Write , { fn ensure_real_path (filename : & FileName) -> & Path { match * filename { FileName :: Real (ref path) => path , _ => panic ! ("cannot format `{filename}` and emit to files") , } } # [allow (non_local_definitions)] impl From < & FileName > for rustc_span :: FileName { fn from (filename : & FileName) -> rustc_span :: FileName { match filename { FileName :: Real (path) => { rustc_span :: FileName :: Real (rustc_span :: RealFileName :: LocalPath (path . to_owned ())) } FileName :: Stdin => rustc_span :: FileName :: Custom ("stdin" . to_owned ()) , } } } let original_text = if newline_style != NewlineStyle :: Auto && * filename != FileName :: Stdin { Arc :: new (fs :: read_to_string (ensure_real_path (filename)) ?) } else { match psess . and_then (| psess | psess . get_original_snippet (filename)) { Some (ori) => ori , None => Arc :: new (fs :: read_to_string (ensure_real_path (filename)) ?) , } } ; let formatted_file = emitter :: FormattedFile { filename , original_text : original_text . as_str () , formatted_text , } ; emitter . emit_formatted_file (out , formatted_file) }
};
}
