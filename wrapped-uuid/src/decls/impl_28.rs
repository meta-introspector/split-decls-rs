macro_rules! deps {
    () => {
        Uuid!();
        Error!();
        NonNilUuid!();
        ErrorKind!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl TryFrom < Uuid > for NonNilUuid { type Error = Error ; # [doc = " Attempts to convert a [`Uuid`] into a [`NonNilUuid`]."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use uuid::{NonNilUuid, Uuid};"] # [doc = " let uuid = Uuid::from_u128(0x0123456789abcdef0123456789abcdef);"] # [doc = " let non_nil = NonNilUuid::try_from(uuid).unwrap();"] # [doc = " ```"] fn try_from (uuid : Uuid) -> Result < Self , Self :: Error > { NonZeroU128 :: new (uuid . as_u128 ()) . map (Self) . ok_or (Error (ErrorKind :: Nil)) } }
    };
}

impl_28!();