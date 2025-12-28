macro_rules! deps {
    () => {
        TestBranch!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'tcx > TestBranch < 'tcx > { fn as_constant (& self) -> Option < ty :: Value < 'tcx > > { if let Self :: Constant (v) = self { Some (* v) } else { None } } }
    };
}

impl_123!()