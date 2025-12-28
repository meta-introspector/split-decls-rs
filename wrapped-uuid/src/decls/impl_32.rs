macro_rules! deps {
    () => {
        Uuid!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl TryFrom < std :: vec :: Vec < u8 > > for Uuid { type Error = Error ; fn try_from (value : std :: vec :: Vec < u8 >) -> Result < Self , Self :: Error > { Uuid :: from_slice (& value) } }
    };
}

impl_32!()