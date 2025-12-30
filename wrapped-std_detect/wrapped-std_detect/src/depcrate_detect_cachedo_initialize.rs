// Generated macro for do_initialize (function)
macro_rules! Depcrate_detect_cachedo_initialize {
() => {
// Module: crate::detect::cache
// Provides: {"do_initialize"}
// Dependencies: {}
# [inline] fn do_initialize (value : Initializer) { CACHE [0] . initialize ((value . 0) as usize & Cache :: MASK) ; CACHE [1] . initialize ((value . 0 >> Cache :: CAPACITY) as usize & Cache :: MASK) ; CACHE [2] . initialize ((value . 0 >> (2 * Cache :: CAPACITY)) as usize & Cache :: MASK) ; }
};
}
