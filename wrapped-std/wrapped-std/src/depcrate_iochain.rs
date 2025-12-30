// Generated macro for Chain (struct)
macro_rules! Depcrate_ioChain {
() => {
// Module: crate::io
// Provides: {"Chain"}
// Dependencies: {}
# [doc = " Adapter to chain together two readers."] # [doc = ""] # [doc = " This struct is generally created by calling [`chain`] on a reader."] # [doc = " Please see the documentation of [`chain`] for more details."] # [doc = ""] # [doc = " [`chain`]: Read::chain"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug)] pub struct Chain < T , U > { first : T , second : U , done_first : bool , }
};
}
