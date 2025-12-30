// Generated macro for write_all_files (function)
macro_rules! Depcrate_source_filewrite_all_files {
() => {
// Module: crate::source_file
// Provides: {"write_all_files"}
// Dependencies: {}
# [cfg (test)] pub (crate) fn write_all_files < T > (source_file : & [FileRecord] , out : & mut T , config : & Config ,) -> Result < () , io :: Error > where T : Write , { let mut emitter = create_emitter (config) ; emitter . emit_header (out) ? ; for (filename , text) in source_file { write_file (None , filename , text , out , & mut * emitter , config . newline_style () ,) ? ; } emitter . emit_footer (out) ? ; Ok (()) }
};
}
