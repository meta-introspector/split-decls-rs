// Generated macro for SwitchWriterFunction (type)
macro_rules! Depcrate_writeSwitchWriterFunction {
() => {
// Module: crate::write
// Provides: {"SwitchWriterFunction"}
// Dependencies: {}
type SwitchWriterFunction < W > = Box < dyn FnOnce (MaybeEncrypted < W >) -> ZipResult < GenericZipWriter < W > > > ;
};
}
