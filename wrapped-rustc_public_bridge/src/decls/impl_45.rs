macro_rules! deps {
    () => {
        Bridge!();
        CompilerCtxt!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > CompilerCtxt < 'tcx , B > { pub fn new (tcx : TyCtxt < 'tcx >) -> Self { Self { tcx , _marker : Default :: default () } } }
    };
}

impl_45!()