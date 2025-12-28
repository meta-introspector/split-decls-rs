macro_rules! deps {
    () => {
        TransformTy!();
        TransformTyOptions!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'tcx > TransformTy < 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx > , options : TransformTyOptions) -> Self { TransformTy { tcx , options , parents : Vec :: new () } } }
    };
}

impl_18!()