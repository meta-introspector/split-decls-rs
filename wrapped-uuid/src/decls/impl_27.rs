macro_rules! deps {
    () => {
        NonNilUuid!();
        Uuid!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl From < NonNilUuid > for Uuid { # [doc = " Converts a [`NonNilUuid`] back into a [`Uuid`]."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use uuid::{NonNilUuid, Uuid};"] # [doc = " let uuid = Uuid::from_u128(0x0123456789abcdef0123456789abcdef);"] # [doc = " let non_nil = NonNilUuid::try_from(uuid).unwrap();"] # [doc = " let uuid_again = Uuid::from(non_nil);"] # [doc = ""] # [doc = " assert_eq!(uuid, uuid_again);"] # [doc = " ```"] fn from (non_nil : NonNilUuid) -> Self { Uuid :: from_u128 (non_nil . 0 . get ()) } }
    };
}

impl_27!();