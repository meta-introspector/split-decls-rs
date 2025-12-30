// Generated macro for create_emitter (function)
macro_rules! Depcratecreate_emitter {
() => {
// Module: crate
// Provides: {"create_emitter"}
// Dependencies: {}
pub (crate) fn create_emitter < 'a > (config : & Config) -> Box < dyn Emitter + 'a > { match config . emit_mode () { EmitMode :: Files if config . make_backup () => { Box :: new (emitter :: FilesWithBackupEmitter :: default ()) } EmitMode :: Files => Box :: new (emitter :: FilesEmitter :: new (config . print_misformatted_file_names () ,)) , EmitMode :: Stdout | EmitMode :: Coverage => { Box :: new (emitter :: StdoutEmitter :: new (config . verbose ())) } EmitMode :: Json => Box :: new (emitter :: JsonEmitter :: default ()) , EmitMode :: ModifiedLines => Box :: new (emitter :: ModifiedLinesEmitter :: default ()) , EmitMode :: Checkstyle => Box :: new (emitter :: CheckstyleEmitter :: default ()) , EmitMode :: Diff => Box :: new (emitter :: DiffEmitter :: new (config . clone ())) , } }
};
}
