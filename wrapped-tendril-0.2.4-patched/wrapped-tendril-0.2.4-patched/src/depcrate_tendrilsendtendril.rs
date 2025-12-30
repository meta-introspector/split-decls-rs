// Generated macro for SendTendril (struct)
macro_rules! Depcrate_tendrilSendTendril {
() => {
// Module: crate::tendril
// Provides: {"SendTendril"}
// Dependencies: {}
# [doc = " A simple wrapper to make `Tendril` `Send`."] # [doc = ""] # [doc = " Although there is a certain subset of the operations on a `Tendril` that a `SendTendril` could"] # [doc = " reasonably implement, in order to clearly separate concerns this type is deliberately"] # [doc = " minimalist, acting as a safe encapsulation around the invariants which permit `Send`ness and"] # [doc = " behaving as an opaque object."] # [doc = ""] # [doc = " A `SendTendril` may be produced by `Tendril.into_send()` or `SendTendril::from(tendril)`,"] # [doc = " and may be returned to a `Tendril` by `Tendril::from(self)`."] pub struct SendTendril < F > where F : fmt :: Format , { tendril : Tendril < F > , }
};
}
