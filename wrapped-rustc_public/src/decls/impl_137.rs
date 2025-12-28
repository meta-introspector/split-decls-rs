macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: CastKind { type T = crate :: mir :: CastKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: CastKind :: * ; match self { PointerExposeProvenance => crate :: mir :: CastKind :: PointerExposeAddress , PointerWithExposedProvenance => crate :: mir :: CastKind :: PointerWithExposedProvenance , PointerCoercion (c , _) => crate :: mir :: CastKind :: PointerCoercion (c . stable (tables , cx)) , IntToInt => crate :: mir :: CastKind :: IntToInt , FloatToInt => crate :: mir :: CastKind :: FloatToInt , FloatToFloat => crate :: mir :: CastKind :: FloatToFloat , IntToFloat => crate :: mir :: CastKind :: IntToFloat , PtrToPtr => crate :: mir :: CastKind :: PtrToPtr , FnPtrToPtr => crate :: mir :: CastKind :: FnPtrToPtr , Transmute => crate :: mir :: CastKind :: Transmute , } } }
    };
}

impl_137!()