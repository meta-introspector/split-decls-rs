// Generated macro for WriteableAsTryWriteableInfallible (struct)
macro_rules! Depcrate_try_writeableWriteableAsTryWriteableInfallible {
() => {
// Module: crate::try_writeable
// Provides: {"WriteableAsTryWriteableInfallible"}
// Dependencies: {}
# [doc = " A wrapper around [`Writeable`] that implements [`TryWriteable`]"] # [doc = " with [`TryWriteable::Error`] set to [`Infallible`]."] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] # [allow (clippy :: exhaustive_structs)] pub struct WriteableAsTryWriteableInfallible < T > (pub T) ;
};
}
