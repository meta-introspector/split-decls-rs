// Generated macro for trailing_slice_layout (function)
macro_rules! Depcratetrailing_slice_layout {
() => {
// Module: crate
// Provides: {"trailing_slice_layout"}
// Dependencies: {}
# [doc = " Efficiently produces the [`TrailingSliceLayout`] of `T`."] # [inline (always)] pub (crate) fn trailing_slice_layout < T > () -> TrailingSliceLayout where T : ? Sized + KnownLayout < PointerMetadata = usize > , { trait LayoutFacts { const SIZE_INFO : TrailingSliceLayout ; } impl < T : ? Sized > LayoutFacts for T where T : KnownLayout < PointerMetadata = usize > , { const SIZE_INFO : TrailingSliceLayout = match T :: LAYOUT . size_info { crate :: SizeInfo :: Sized { .. } => const_panic ! ("unreachable") , crate :: SizeInfo :: SliceDst (info) => info , } ; } T :: SIZE_INFO }
};
}
