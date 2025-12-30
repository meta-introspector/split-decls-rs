// Generated macro for impl_99 (impl)
macro_rules! Depcrate_directoryimpl_99 {
() => {
// Module: crate::directory
// Provides: {"impl_99"}
// Dependencies: {}
impl Directory { pub fn new < P : Into < PathBuf > > (path : P) -> Self { let mut path = path . into () ; path . push ("") ; Directory { path } } pub fn current () -> io :: Result < Self > { env :: current_dir () . map (Directory :: new) } pub fn to_string_lossy (& self) -> Cow < str > { self . path . to_string_lossy () } pub fn join < P : AsRef < Path > > (& self , tail : P) -> PathBuf { self . path . join (tail) } pub fn parent (& self) -> Option < Self > { self . path . parent () . map (Directory :: new) } pub fn canonicalize (& self) -> io :: Result < Self > { self . path . canonicalize () . map (Directory :: new) } }
};
}
