// Generated macro for std (module)
macro_rules! Depcratestd {
() => {
// Module: crate
// Provides: {"std"}
// Dependencies: {}
# [cfg (all (not (test) , feature = "alloc" , not (feature = "std")))] # [allow (unused_imports)] mod std { pub use crate :: { alloc :: { borrow , boxed , string , vec } , core :: * , } ; # [cfg (feature = "owned")] pub use crate :: alloc :: sync ; }
};
}
