// Generated macro for impl_103 (impl)
macro_rules! Depcrate_intimpl_103 {
() => {
// Module: crate::int
// Provides: {"impl_103"}
// Dependencies: {}
impl < M , N > PartialDiv < N > for M where M : Integer + Div < N > + Rem < N , Output = Z0 > , { type Output = Quot < M , N > ; # [inline] fn partial_div (self , rhs : N) -> Self :: Output { self / rhs } }
};
}
