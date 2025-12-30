// Generated macro for impl_fmt_traits (macro)
macro_rules! Depcrate_macrosimpl_fmt_traits {
() => {
// Module: crate::macros
// Provides: {"impl_fmt_traits"}
// Dependencies: {}
macro_rules ! impl_fmt_traits { ($ ($ t : tt) *) => { impl_fmt_trait ! (core :: fmt :: Display , "{}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: Debug , "{:?}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: Octal , "{:o}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: LowerHex , "{:x}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: UpperHex , "{:X}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: Pointer , "{:p}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: Binary , "{:b}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: LowerExp , "{:e}" $ ($ t) *) ; impl_fmt_trait ! (core :: fmt :: UpperExp , "{:E}" $ ($ t) *) ; } ; }
};
}
