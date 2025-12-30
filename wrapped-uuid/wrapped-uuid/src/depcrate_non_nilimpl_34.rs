// Generated macro for impl_34 (impl)
macro_rules! Depcrate_non_nilimpl_34 {
() => {
// Module: crate::non_nil
// Provides: {"impl_34"}
// Dependencies: {}
impl TryFrom < Uuid > for NonNilUuid { type Error = Error ; # [doc = " Attempts to convert a [`Uuid`] into a [`NonNilUuid`]."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use uuid::{NonNilUuid, Uuid};"] # [doc = " let uuid = Uuid::from_u128(0x0123456789abcdef0123456789abcdef);"] # [doc = " let non_nil = NonNilUuid::try_from(uuid).unwrap();"] # [doc = " ```"] fn try_from (uuid : Uuid) -> Result < Self , Self :: Error > { NonZeroU128 :: new (uuid . as_u128 ()) . map (Self) . ok_or (Error (ErrorKind :: Nil)) } }
};
}
