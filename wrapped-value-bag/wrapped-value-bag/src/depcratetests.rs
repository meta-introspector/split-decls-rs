// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: std :: mem ; # [cfg (feature = "inline-i128")] const SIZE_LIMIT_U64 : usize = 4 ; # [cfg (not (feature = "inline-i128"))] const SIZE_LIMIT_U64 : usize = 3 ; # [test] fn value_bag_size () { let size = mem :: size_of :: < ValueBag < '_ > > () ; let limit = mem :: size_of :: < u64 > () * SIZE_LIMIT_U64 ; if size > limit { panic ! ("`ValueBag` size ({} bytes) is too large (expected up to {} bytes)" , size , limit ,) ; } } }
};
}
