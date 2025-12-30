// Generated macro for impl_65 (impl)
macro_rules! Depcrate_reprimpl_65 {
() => {
// Module: crate::repr
// Provides: {"impl_65"}
// Dependencies: {}
impl < Prim , Packed > Repr < Prim , Packed > { pub (crate) fn from_attrs (attrs : & [Attribute]) -> Result < Repr < Prim , Packed > , Error > where Prim : With < PrimitiveRepr > , Packed : With < NonZeroU32 > , { Repr :: from_attrs_inner (attrs) . map_err (Into :: into) } }
};
}
