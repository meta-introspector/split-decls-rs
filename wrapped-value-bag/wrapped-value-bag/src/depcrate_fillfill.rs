// Generated macro for Fill (trait)
macro_rules! Depcrate_fillFill {
() => {
// Module: crate::fill
// Provides: {"Fill"}
// Dependencies: {}
# [doc = " A type that requires extra work to convert into a [`ValueBag`](../struct.ValueBag.html)."] # [doc = ""] # [doc = " This trait is an advanced initialization API."] # [doc = " It's intended for erased values coming from other logging frameworks that may need"] # [doc = " to perform extra work to determine the concrete type to use."] pub trait Fill { # [doc = " Fill a value."] fn fill (& self , slot : Slot) -> Result < () , Error > ; }
};
}
