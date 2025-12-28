macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [cfg (feature = "pem")] impl From < pem :: Error > for Error { fn from (err : pem :: Error) -> Error { der :: Error :: from (err) . into () } }
    };
}

impl_36!()