macro_rules! deps {
    () => {
        SliceDst!();
        KnownLayout!();
        TrailingSliceLayout!();
        SizeInfo!();
        PointerMetadata!();
    };
}

macro_rules! trailing_slice_layout {
    () => {
        deps!();
        # [doc = " Efficiently produces the [`TrailingSliceLayout`] of `T`."] # [inline (always)] pub (crate) fn trailing_slice_layout < T > () -> TrailingSliceLayout where T : ? Sized + KnownLayout < PointerMetadata = usize > , { trait LayoutFacts { const SIZE_INFO : TrailingSliceLayout ; } impl < T : ? Sized > LayoutFacts for T where T : KnownLayout < PointerMetadata = usize > , { const SIZE_INFO : TrailingSliceLayout = match T :: LAYOUT . size_info { crate :: SizeInfo :: Sized { .. } => const_panic ! ("unreachable") , crate :: SizeInfo :: SliceDst (info) => info , } ; } T :: SIZE_INFO }
    };
}

trailing_slice_layout!();