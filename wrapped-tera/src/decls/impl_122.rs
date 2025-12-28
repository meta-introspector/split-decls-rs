macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl From < serde_json :: Error > for Error { fn from (e : serde_json :: Error) -> Self { Self :: json (e) } }
    };
}

impl_122!();