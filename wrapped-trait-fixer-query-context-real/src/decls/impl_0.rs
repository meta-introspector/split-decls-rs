macro_rules! impl_0 {
    () => {
        impl < 'tcx > QueryContext < 'tcx , TyCtxt < 'tcx > , Item < 'tcx > > for TyCtxt < 'tcx > { fn walk_hir_tops (& self , mut f : impl FnMut (& 'tcx Item < 'tcx >)) { self . hir () . walk_tops (| item | f (item)) ; } }
    };
}

impl_0!()