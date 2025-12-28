macro_rules! deps {
    () => {
        TransformTyOptions!();
    };
}

macro_rules! TransformTy {
    () => {
        deps!();
        pub (crate) struct TransformTy < 'tcx > { tcx : TyCtxt < 'tcx > , options : TransformTyOptions , parents : Vec < Ty < 'tcx > > , }
    };
}

TransformTy!();