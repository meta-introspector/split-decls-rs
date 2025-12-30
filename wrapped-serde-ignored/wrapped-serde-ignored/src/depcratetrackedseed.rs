// Generated macro for TrackedSeed (struct)
macro_rules! DepcrateTrackedSeed {
() => {
// Module: crate
// Provides: {"TrackedSeed"}
// Dependencies: {}
# [doc = " Seed used for map values, sequence elements and newtype variants to track"] # [doc = " their path."] struct TrackedSeed < 'a , X , F : 'a > { seed : X , callback : & 'a mut F , path : Path < 'a > , }
};
}
