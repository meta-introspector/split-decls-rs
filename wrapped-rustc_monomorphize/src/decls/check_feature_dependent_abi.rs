macro_rules! check_feature_dependent_abi {
    () => {
        pub (crate) fn check_feature_dependent_abi < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > ,) { check_instance_abi (tcx , instance) ; check_callees_abi (tcx , instance , body) ; }
    };
}

check_feature_dependent_abi!()