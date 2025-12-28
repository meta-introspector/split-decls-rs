macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: StatementKind < 'tcx > { type T = crate :: mir :: StatementKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { mir :: StatementKind :: Assign (assign) => crate :: mir :: StatementKind :: Assign (assign . 0 . stable (tables , cx) , assign . 1 . stable (tables , cx) ,) , mir :: StatementKind :: FakeRead (fake_read_place) => crate :: mir :: StatementKind :: FakeRead (fake_read_place . 0 . stable (tables , cx) , fake_read_place . 1 . stable (tables , cx) ,) , mir :: StatementKind :: SetDiscriminant { place , variant_index } => { crate :: mir :: StatementKind :: SetDiscriminant { place : place . as_ref () . stable (tables , cx) , variant_index : variant_index . stable (tables , cx) , } } mir :: StatementKind :: Deinit (place) => { crate :: mir :: StatementKind :: Deinit (place . stable (tables , cx)) } mir :: StatementKind :: StorageLive (place) => { crate :: mir :: StatementKind :: StorageLive (place . stable (tables , cx)) } mir :: StatementKind :: StorageDead (place) => { crate :: mir :: StatementKind :: StorageDead (place . stable (tables , cx)) } mir :: StatementKind :: Retag (retag , place) => { crate :: mir :: StatementKind :: Retag (retag . stable (tables , cx) , place . stable (tables , cx)) } mir :: StatementKind :: PlaceMention (place) => { crate :: mir :: StatementKind :: PlaceMention (place . stable (tables , cx)) } mir :: StatementKind :: AscribeUserType (place_projection , variance) => { crate :: mir :: StatementKind :: AscribeUserType { place : place_projection . as_ref () . 0 . stable (tables , cx) , projections : place_projection . as_ref () . 1 . stable (tables , cx) , variance : variance . stable (tables , cx) , } } mir :: StatementKind :: Coverage (coverage) => { crate :: mir :: StatementKind :: Coverage (opaque (coverage)) } mir :: StatementKind :: Intrinsic (intrinstic) => { crate :: mir :: StatementKind :: Intrinsic (intrinstic . stable (tables , cx)) } mir :: StatementKind :: ConstEvalCounter => crate :: mir :: StatementKind :: ConstEvalCounter , mir :: StatementKind :: BackwardIncompatibleDropHint { .. } => { crate :: mir :: StatementKind :: Nop } mir :: StatementKind :: Nop => crate :: mir :: StatementKind :: Nop , } } }
    };
}

impl_129!()