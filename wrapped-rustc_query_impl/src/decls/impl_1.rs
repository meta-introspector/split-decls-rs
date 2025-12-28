macro_rules! deps {
    () => {
        QueryCtxt!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'tcx > QueryCtxt < 'tcx > { # [inline] pub fn new (tcx : TyCtxt < 'tcx >) -> Self { QueryCtxt { tcx } } }
    };
}

impl_1!()