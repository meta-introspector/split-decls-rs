macro_rules! deps {
    () => {
        EcParameters!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'a > From < & 'a EcParameters > for AnyRef < 'a > { fn from (params : & 'a EcParameters) -> AnyRef < 'a > { match params { EcParameters :: NamedCurve (oid) => oid . into () , } } }
    };
}

impl_42!();