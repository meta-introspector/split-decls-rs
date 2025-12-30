// Generated macro for alloc_align (function)
macro_rules! Depcratealloc_align {
() => {
// Module: crate
// Provides: {"alloc_align"}
// Dependencies: {}
# [doc = " Gets the align necessary to allocate a `ThinVec<T>`"] fn alloc_align < T > () -> usize { max (mem :: align_of :: < T > () , mem :: align_of :: < Header > ()) }
};
}
