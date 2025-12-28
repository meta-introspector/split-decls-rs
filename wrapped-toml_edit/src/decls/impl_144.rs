macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        # [doc (hidden)] impl From < Key > for String { fn from (key : Key) -> Self { key . key } }
    };
}

impl_144!()