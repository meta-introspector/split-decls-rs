macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: NullOp < 'tcx > { type T = crate :: mir :: NullOp ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: NullOp :: * ; match self { SizeOf => crate :: mir :: NullOp :: SizeOf , AlignOf => crate :: mir :: NullOp :: AlignOf , OffsetOf (indices) => crate :: mir :: NullOp :: OffsetOf (indices . iter () . map (| idx | idx . stable (tables , cx)) . collect () ,) , UbChecks => crate :: mir :: NullOp :: UbChecks , ContractChecks => crate :: mir :: NullOp :: ContractChecks , } } }
    };
}

impl_136!()