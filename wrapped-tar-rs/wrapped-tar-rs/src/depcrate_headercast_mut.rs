// Generated macro for cast_mut (function)
macro_rules! Depcrate_headercast_mut {
() => {
// Module: crate::header
// Provides: {"cast_mut"}
// Dependencies: {}
unsafe fn cast_mut < T , U > (a : & mut T) -> & mut U { assert_eq ! (mem :: size_of_val (a) , mem :: size_of ::< U > ()) ; assert_eq ! (mem :: align_of_val (a) , mem :: align_of ::< U > ()) ; & mut * (a as * mut T as * mut U) }
};
}
