// Generated macro for impl_486 (impl)
macro_rules! Depcrateimpl_486 {
() => {
// Module: crate
// Provides: {"impl_486"}
// Dependencies: {}
impl PointerMetadata for () { # [inline] # [allow (clippy :: unused_unit)] fn from_elem_count (_elems : usize) -> () { } # [inline] fn size_for_metadata (self , layout : DstLayout) -> Option < usize > { match layout . size_info { SizeInfo :: Sized { size } => Some (size) , SizeInfo :: SliceDst (_) => None , } } }
};
}
