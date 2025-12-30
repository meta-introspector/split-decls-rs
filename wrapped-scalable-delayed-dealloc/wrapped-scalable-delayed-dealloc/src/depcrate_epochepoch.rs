// Generated macro for Epoch (struct)
macro_rules! Depcrate_epochEpoch {
() => {
// Module: crate::epoch
// Provides: {"Epoch"}
// Dependencies: {}
# [doc = " [`Epoch`] is a unit of time that dictates the lifetime of retired memory regions."] # [doc = ""] # [doc = " The global epoch rotates `64` [`Epoch`] values in a range of `[0..63]`, instead of monotonically"] # [doc = " increasing to reduce the memory footprint."] # [derive (Clone , Copy , Debug , Default , Eq , Ord , PartialEq , PartialOrd)] pub struct Epoch { value : u8 , }
};
}
