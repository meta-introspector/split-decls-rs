// Generated macro for impl_21 (impl)
macro_rules! Depcrate_constructorimpl_21 {
() => {
// Module: crate::constructor
// Provides: {"impl_21"}
// Dependencies: {}
impl fmt :: Display for RangeEnd { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { RangeEnd :: Included => "..=" , RangeEnd :: Excluded => ".." , }) } }
};
}
