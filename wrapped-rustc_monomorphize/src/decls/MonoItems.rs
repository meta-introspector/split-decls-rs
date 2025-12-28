macro_rules! MonoItems {
    () => {
        struct MonoItems < 'tcx > { items : FxIndexMap < MonoItem < 'tcx > , Span > , }
    };
}

MonoItems!()