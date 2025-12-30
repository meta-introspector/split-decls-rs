// Generated macro for impl_67 (impl)
macro_rules! Depcrate_abiimpl_67 {
() => {
// Module: crate::abi
// Provides: {"impl_67"}
// Dependencies: {}
impl Primitive { pub fn size (self , target : & MachineInfo) -> Size { match self { Primitive :: Int { length , .. } => Size :: from_bits (length . bits ()) , Primitive :: Float { length } => Size :: from_bits (length . bits ()) , Primitive :: Pointer (_) => target . pointer_width , } } }
};
}
