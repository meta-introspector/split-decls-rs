// Generated macro for PrivateInvert (trait)
macro_rules! Depcrate_privatePrivateInvert {
() => {
// Module: crate::private
// Provides: {"PrivateInvert"}
// Dependencies: {}
# [doc = " Doubly private! Called by invert to make the magic happen once its done the first step."] # [doc = " The Rhs is what we've got so far."] pub trait PrivateInvert < Rhs > { type Output ; fn private_invert (self , rhs : Rhs) -> Self :: Output ; }
};
}
