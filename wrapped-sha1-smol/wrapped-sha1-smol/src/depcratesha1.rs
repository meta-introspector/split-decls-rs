// Generated macro for Sha1 (struct)
macro_rules! DepcrateSha1 {
() => {
// Module: crate
// Provides: {"Sha1"}
// Dependencies: {}
# [doc = " Represents a Sha1 hash object in memory."] # [derive (Clone , PartialOrd , Ord , PartialEq , Eq , Hash)] pub struct Sha1 { state : Sha1State , blocks : Blocks , len : u64 , }
};
}
