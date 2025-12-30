// Generated macro for stripes_with_tail (function)
macro_rules! Depcrate_xxhash3stripes_with_tail {
() => {
// Module: crate::xxhash3
// Provides: {"stripes_with_tail"}
// Dependencies: {}
# [inline] pub fn stripes_with_tail (block : & [u8]) -> (& [[u8 ; 64]] , & [u8]) { match block . bp_as_chunks () { ([stripes @ .. , last] , []) => (stripes , last) , (stripes , last) => (stripes , last) , } }
};
}
