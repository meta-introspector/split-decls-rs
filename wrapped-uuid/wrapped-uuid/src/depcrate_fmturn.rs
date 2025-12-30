// Generated macro for Urn (struct)
macro_rules! Depcrate_fmtUrn {
() => {
// Module: crate::fmt
// Provides: {"Urn"}
// Dependencies: {}
# [doc = " Format a [`Uuid`] as a URN string, like"] # [doc = " `urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8`."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , Ord , PartialEq , PartialOrd)] # [cfg_attr (all (uuid_unstable , feature = "zerocopy") , derive (zerocopy :: IntoBytes , zerocopy :: FromBytes , zerocopy :: KnownLayout , zerocopy :: Immutable , zerocopy :: Unaligned))] # [repr (transparent)] pub struct Urn (Uuid) ;
};
}
