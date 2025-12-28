macro_rules! deps {
    () => {
        Bridge!();
        CompilerCtxt!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > HasTyCtxt < 'tcx > for CompilerCtxt < 'tcx , B > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
    };
}

impl_49!();