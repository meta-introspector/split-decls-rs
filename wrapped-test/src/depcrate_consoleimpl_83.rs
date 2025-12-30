// Generated macro for impl_83 (impl)
macro_rules! Depcrate_consoleimpl_83 {
() => {
// Module: crate::console
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : Write > Write for OutputLocation < T > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { match * self { OutputLocation :: Pretty (ref mut term) => term . write (buf) , OutputLocation :: Raw (ref mut stdout) => stdout . write (buf) , } } fn flush (& mut self) -> io :: Result < () > { match * self { OutputLocation :: Pretty (ref mut term) => term . flush () , OutputLocation :: Raw (ref mut stdout) => stdout . flush () , } } }
};
}
