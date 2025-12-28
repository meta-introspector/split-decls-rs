macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < Uuid > for String { fn from (uuid : Uuid) -> Self { uuid . to_string () } }
    };
}

impl_46!()