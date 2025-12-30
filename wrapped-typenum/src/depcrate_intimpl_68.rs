// Generated macro for impl_68 (impl)
macro_rules! Depcrate_intimpl_68 {
() => {
// Module: crate::int
// Provides: {"impl_68"}
// Dependencies: {}
# [doc = " `-PInt = NInt`"] impl < U : Unsigned + NonZero > Neg for PInt < U > { type Output = NInt < U > ; # [inline] fn neg (self) -> Self :: Output { NInt :: new () } }
};
}
