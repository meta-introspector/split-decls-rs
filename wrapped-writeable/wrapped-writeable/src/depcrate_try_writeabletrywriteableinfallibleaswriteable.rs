// Generated macro for TryWriteableInfallibleAsWriteable (struct)
macro_rules! Depcrate_try_writeableTryWriteableInfallibleAsWriteable {
() => {
// Module: crate::try_writeable
// Provides: {"TryWriteableInfallibleAsWriteable"}
// Dependencies: {}
# [doc = " A wrapper around [`TryWriteable`] that implements [`Writeable`]"] # [doc = " if [`TryWriteable::Error`] is [`Infallible`]."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] # [allow (clippy :: exhaustive_structs)] pub struct TryWriteableInfallibleAsWriteable < T > (pub T) ;
};
}
