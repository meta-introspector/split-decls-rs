// Generated macro for wrapped_payload (macro)
macro_rules! Depcrate_msgs_handshakewrapped_payload {
() => {
// Module: crate::msgs::handshake
// Provides: {"wrapped_payload"}
// Dependencies: {}
# [doc = " Create a newtype wrapper around a given type."] # [doc = ""] # [doc = " This is used to create newtypes for the various TLS message types which is used to wrap"] # [doc = " the `PayloadU8` or `PayloadU16` types. This is typically used for types where we don't need"] # [doc = " anything other than access to the underlying bytes."] macro_rules ! wrapped_payload (($ (# [$ comment : meta]) * $ vis : vis struct $ name : ident , $ inner : ident $ (<$ inner_ty : ty >) ?,) => { $ (# [$ comment]) * # [derive (Clone , Debug)] $ vis struct $ name ($ inner $ (<$ inner_ty >) ?) ; impl From < Vec < u8 >> for $ name { fn from (v : Vec < u8 >) -> Self { Self ($ inner :: new (v)) } } impl AsRef < [u8] > for $ name { fn as_ref (& self) -> & [u8] { self . 0.0 . as_slice () } } impl Codec <'_ > for $ name { fn encode (& self , bytes : & mut Vec < u8 >) { self . 0 . encode (bytes) ; } fn read (r : & mut Reader <'_ >) -> Result < Self , InvalidMessage > { Ok (Self ($ inner :: read (r) ?)) } } }) ;
};
}
