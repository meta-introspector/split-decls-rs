// Generated macro for std (module)
macro_rules! Depcratestd {
() => {
// Module: crate
// Provides: {"std"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , not (feature = "std")))] mod std { pub use crate :: { alloc :: { borrow , boxed , collections , string , vec } , core :: { cmp , convert , fmt , hash , marker , mem , ops , result , str , write } , } ; }
};
}
