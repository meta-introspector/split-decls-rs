// Generated macro for Tai64N (struct)
macro_rules! DepcrateTai64N {
() => {
// Module: crate
// Provides: {"Tai64N"}
// Dependencies: {}
# [doc = " A `TAI64N` timestamp."] # [doc = ""] # [doc = " Invariant: The nanosecond part <= 999999999."] # [derive (Copy , Clone , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct Tai64N (pub Tai64 , pub u32) ;
};
}
