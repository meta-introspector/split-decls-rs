// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (target_pointer_width = "64")] mod tests { use super :: * ; use core :: mem :: size_of ; # [doc = " Checks that the size of the type is one of the given sizes."] # [doc = " The size might differ across Rust versions or channels."] macro_rules ! check_size_of { ($ sizes : pat , $ type : path) => { assert ! (matches ! (size_of ::<$ type > () , $ sizes) , concat ! (stringify ! ($ type) , " is of size {}") , size_of ::<$ type > ()) ; } ; } # [test] fn check_sizes () { check_size_of ! (24 , ZeroVec < u8 >) ; check_size_of ! (24 , ZeroVec < u32 >) ; check_size_of ! (32 | 24 , VarZeroVec < [u8] >) ; check_size_of ! (32 | 24 , VarZeroVec < str >) ; check_size_of ! (48 , ZeroMap < u32 , u32 >) ; check_size_of ! (56 | 48 , ZeroMap < u32 , str >) ; check_size_of ! (56 | 48 , ZeroMap < str , u32 >) ; check_size_of ! (64 | 48 , ZeroMap < str , str >) ; check_size_of ! (120 | 96 , ZeroMap2d < str , str , str >) ; check_size_of ! (24 , Option < ZeroVec < u8 >>) ; check_size_of ! (32 | 24 , Option < VarZeroVec < str >>) ; check_size_of ! (64 | 56 | 48 , Option < ZeroMap < str , str >>) ; check_size_of ! (120 | 104 | 96 , Option < ZeroMap2d < str , str , str >>) ; } }
};
}
