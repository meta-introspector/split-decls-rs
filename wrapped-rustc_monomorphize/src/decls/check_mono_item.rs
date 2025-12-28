macro_rules! check_mono_item {
    () => {
        fn check_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) { let body = tcx . instance_mir (instance . def) ; abi_check :: check_feature_dependent_abi (tcx , instance , body) ; move_check :: check_moves (tcx , instance , body) ; }
    };
}

check_mono_item!()