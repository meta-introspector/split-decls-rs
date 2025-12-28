macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl From < der :: Error > for Error { fn from (err : der :: Error) -> Error { Error :: Asn1 (err) } }
    };
}

impl_35!()