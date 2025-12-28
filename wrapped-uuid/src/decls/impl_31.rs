macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < Uuid > for std :: vec :: Vec < u8 > { fn from (value : Uuid) -> Self { value . 0 . to_vec () } }
    };
}

impl_31!()