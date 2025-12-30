// Generated macro for format_file (function)
macro_rules! Depcrate_testformat_file {
() => {
// Module: crate::test
// Provides: {"format_file"}
// Dependencies: {}
fn format_file < P : Into < PathBuf > > (filepath : P , config : Config) -> (bool , SourceFile , FormatReport) { let filepath = filepath . into () ; let input = Input :: File (filepath) ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; let result = session . format (input) . unwrap () ; let parsing_errors = session . has_parsing_errors () ; let mut source_file = SourceFile :: new () ; mem :: swap (& mut session . source_file , & mut source_file) ; (parsing_errors , source_file , result) }
};
}
