macro_rules! deps {
    () => {
        Interner!();
        VarianceDiagInfo!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < I : Interner > VarianceDiagInfo < I > { # [doc = " Mirrors `Variance::xform` - used to 'combine' the existing"] # [doc = " and new `VarianceDiagInfo`s when our variance changes."] pub fn xform (self , other : VarianceDiagInfo < I >) -> VarianceDiagInfo < I > { match self { VarianceDiagInfo :: None => other , VarianceDiagInfo :: Invariant { .. } => self , } } }
    };
}

impl_112!();