// Generated macro for cast (function)
macro_rules! Depcrate_headercast {
() => {
// Module: crate::header
// Provides: {"cast"}
// Dependencies: {}
unsafe fn cast < T , U > (a : & T) -> & U { assert_eq ! (mem :: size_of_val (a) , mem :: size_of ::< U > ()) ; assert_eq ! (mem :: align_of_val (a) , mem :: align_of ::< U > ()) ; & * (a as * const T as * const U) }
};
}
