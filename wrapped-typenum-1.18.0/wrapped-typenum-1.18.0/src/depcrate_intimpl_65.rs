// Generated macro for impl_65 (impl)
macro_rules! Depcrate_intimpl_65 {
() => {
// Module: crate::int
// Provides: {"impl_65"}
// Dependencies: {}
# [doc = " `-PInt = NInt`"] impl < U : Unsigned + NonZero > Neg for PInt < U > { type Output = NInt < U > ; # [inline] fn neg (self) -> Self :: Output { NInt :: new () } }
};
}
