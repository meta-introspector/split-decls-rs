// Generated macro for impl_775 (impl)
macro_rules! Depcrate_specimpl_775 {
() => {
// Module: crate::spec
// Provides: {"impl_775"}
// Dependencies: {}
impl BinaryFormat { # [doc = " Returns [`object::BinaryFormat`] for given `BinaryFormat`"] pub fn to_object (& self) -> object :: BinaryFormat { match self { Self :: Coff => object :: BinaryFormat :: Coff , Self :: Elf => object :: BinaryFormat :: Elf , Self :: MachO => object :: BinaryFormat :: MachO , Self :: Wasm => object :: BinaryFormat :: Wasm , Self :: Xcoff => object :: BinaryFormat :: Xcoff , } } }
};
}
