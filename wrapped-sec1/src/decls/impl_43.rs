macro_rules! deps {
    () => {
        EcParameters!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl From < ObjectIdentifier > for EcParameters { fn from (oid : ObjectIdentifier) -> EcParameters { EcParameters :: NamedCurve (oid) } }
    };
}

impl_43!()