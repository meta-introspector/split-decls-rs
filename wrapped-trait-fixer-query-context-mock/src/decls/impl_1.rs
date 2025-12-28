macro_rules! deps {
    () => {
        MockTyCtxt!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'tcx > QueryContext < 'tcx , TyCtxt < 'tcx > , Item < 'tcx > > for MockTyCtxt < 'tcx > { fn walk_hir_tops (& self , _f : impl FnMut (& 'tcx Item < 'tcx >)) { println ! ("Mock MockTyCtxt::walk_hir_tops called") ; } }
    };
}

impl_1!()