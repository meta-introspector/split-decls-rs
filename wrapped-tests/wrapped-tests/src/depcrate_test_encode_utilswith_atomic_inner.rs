// Generated macro for WITH_ATOMIC_INNER (const)
macro_rules! Depcrate_test_encode_utilsWITH_ATOMIC_INNER {
() => {
// Module: crate::test_encode_utils
// Provides: {"WITH_ATOMIC_INNER"}
// Dependencies: {}
const WITH_ATOMIC_INNER : Encoding = Encoding :: Struct ("with_atomic_inner" , & [AtomicI32 :: ENCODING , < * const AtomicI32 > :: ENCODING , < AtomicPtr < c_int > > :: ENCODING ,] ,) ;
};
}
