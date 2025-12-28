macro_rules! deps {
    () => {
        RevealedTy!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'tcx > RevealedTy < 'tcx > { pub fn inner (self) -> Ty < 'tcx > { self . 0 } }
    };
}

impl_76!();