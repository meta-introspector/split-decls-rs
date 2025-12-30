// Generated macro for impl_241 (impl)
macro_rules! Depcrate_hedge_selectimpl_241 {
() => {
// Module: crate::hedge::select
// Provides: {"impl_241"}
// Dependencies: {}
impl < P , A , B > Select < P , A , B > { pub const fn new < Request > (policy : P , a : A , b : B) -> Self where P : Policy < Request > , A : Service < Request > , A :: Error : Into < crate :: BoxError > , B : Service < Request , Response = A :: Response > , B :: Error : Into < crate :: BoxError > , { Select { policy , a , b } } }
};
}
