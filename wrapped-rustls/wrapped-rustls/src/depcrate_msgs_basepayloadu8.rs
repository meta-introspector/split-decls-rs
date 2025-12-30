// Generated macro for PayloadU8 (struct)
macro_rules! Depcrate_msgs_basePayloadU8 {
() => {
// Module: crate::msgs::base
// Provides: {"PayloadU8"}
// Dependencies: {}
# [doc = " An arbitrary, unknown-content, u8-length-prefixed payload"] # [doc = ""] # [doc = " `C` controls the minimum length accepted when decoding."] # [derive (Clone , Eq , PartialEq)] pub (crate) struct PayloadU8 < C : Cardinality = MaybeEmpty > (pub (crate) Vec < u8 > , PhantomData < C >) ;
};
}
