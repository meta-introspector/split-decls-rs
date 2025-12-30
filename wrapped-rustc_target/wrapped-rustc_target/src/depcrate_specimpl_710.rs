// Generated macro for impl_710 (impl)
macro_rules! Depcrate_specimpl_710 {
() => {
// Module: crate::spec
// Provides: {"impl_710"}
// Dependencies: {}
impl LinkerFlavorCli { # [doc = " Returns whether this `-C linker-flavor` option is one of the unstable values."] pub fn is_unstable (& self) -> bool { match self { LinkerFlavorCli :: Gnu (..) | LinkerFlavorCli :: Darwin (..) | LinkerFlavorCli :: WasmLld (..) | LinkerFlavorCli :: Unix (..) | LinkerFlavorCli :: Msvc (Lld :: Yes) | LinkerFlavorCli :: EmCc | LinkerFlavorCli :: Bpf | LinkerFlavorCli :: Llbc | LinkerFlavorCli :: Ptx => true , LinkerFlavorCli :: Gcc | LinkerFlavorCli :: Ld | LinkerFlavorCli :: Lld (..) | LinkerFlavorCli :: Msvc (Lld :: No) | LinkerFlavorCli :: Em => false , } } }
};
}
