macro_rules! deps {
    () => {
        EcParameters!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl EcParameters { # [doc = " Obtain the `namedCurve` OID."] pub fn named_curve (self) -> Option < ObjectIdentifier > { match self { Self :: NamedCurve (oid) => Some (oid) , } } }
    };
}

impl_41!()