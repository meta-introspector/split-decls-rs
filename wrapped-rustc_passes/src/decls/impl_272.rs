macro_rules! deps {
    () => {
        UnwrapLayoutCx!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < 'tcx > HasTyCtxt < 'tcx > for UnwrapLayoutCx < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
    };
}

impl_272!()