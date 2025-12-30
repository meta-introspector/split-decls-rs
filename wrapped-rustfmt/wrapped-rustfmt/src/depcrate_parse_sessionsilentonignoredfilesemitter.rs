// Generated macro for SilentOnIgnoredFilesEmitter (struct)
macro_rules! Depcrate_parse_sessionSilentOnIgnoredFilesEmitter {
() => {
// Module: crate::parse::session
// Provides: {"SilentOnIgnoredFilesEmitter"}
// Dependencies: {}
# [doc = " Emit errors against every files expect ones specified in the `ignore_path_set`."] struct SilentOnIgnoredFilesEmitter { ignore_path_set : IntoDynSyncSend < Arc < IgnorePathSet > > , source_map : Arc < SourceMap > , emitter : Box < DynEmitter > , has_non_ignorable_parser_errors : bool , can_reset : Arc < AtomicBool > , }
};
}
