macro_rules! deps {
    () => {
        Error!();
        Uuid!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl TryFrom < String > for Uuid { type Error = Error ; fn try_from (uuid_str : String) -> Result < Self , Self :: Error > { Uuid :: try_from (uuid_str . as_ref ()) } }
    };
}

impl_33!();