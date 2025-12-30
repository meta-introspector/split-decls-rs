// Generated macro for impl_135 (impl)
macro_rules! Depcrate_headerimpl_135 {
() => {
// Module: crate::header
// Provides: {"impl_135"}
// Dependencies: {}
impl < H , T > HeaderSliceWithLengthProtected < H , T > { pub fn header (& self) -> & H { & self . inner . header . header } pub fn header_mut (& mut self) -> & mut H { & mut self . inner . header . header } pub fn length (& self) -> usize { self . inner . header . length } pub fn slice (& self) -> & [T] { & self . inner . slice } pub fn slice_mut (& mut self) -> & mut [T] { & mut self . inner . slice } pub (crate) fn inner (& self) -> & HeaderSliceWithLengthUnchecked < H , T > { & self . inner } }
};
}
