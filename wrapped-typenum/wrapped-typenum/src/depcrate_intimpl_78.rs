// Generated macro for impl_78 (impl)
macro_rules! Depcrate_intimpl_78 {
() => {
// Module: crate::int
// Provides: {"impl_78"}
// Dependencies: {}
# [doc = " `P + N = Positive` where `P > N`"] impl < N : Unsigned , P : Unsigned > PrivateIntegerAdd < Greater , N > for P where P : Sub < N > , < P as Sub < N > > :: Output : Unsigned + NonZero , { type Output = PInt < < P as Sub < N > > :: Output > ; # [inline] fn private_integer_add (self , _ : Greater , n : N) -> Self :: Output { PInt { n : self - n } } }
};
}
