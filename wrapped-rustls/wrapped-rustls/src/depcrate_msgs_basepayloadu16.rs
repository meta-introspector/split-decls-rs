// Generated macro for PayloadU16 (struct)
macro_rules! Depcrate_msgs_basePayloadU16 {
() => {
// Module: crate::msgs::base
// Provides: {"PayloadU16"}
// Dependencies: {}
# [doc = " An arbitrary, unknown-content, u16-length-prefixed payload"] # [doc = ""] # [doc = " The `C` type parameter controls whether decoded values may"] # [doc = " be empty."] # [derive (Clone , Eq , PartialEq)] pub (crate) struct PayloadU16 < C : Cardinality = MaybeEmpty > (pub (crate) Vec < u8 > , PhantomData < C >) ;
};
}
