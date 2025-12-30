// Generated macro for impl_1289 (impl)
macro_rules! Depcrate_mock_commandimpl_1289 {
() => {
// Module: crate::mock_command
// Provides: {"impl_1289"}
// Dependencies: {}
impl AsyncCommand { pub fn new < S : AsRef < OsStr > > (program : S , jobserver : Client) -> AsyncCommand { AsyncCommand { inner : Some (Command :: new (program)) , jobserver , } } fn inner (& mut self) -> & mut Command { self . inner . as_mut () . expect ("can't reuse commands") } }
};
}
