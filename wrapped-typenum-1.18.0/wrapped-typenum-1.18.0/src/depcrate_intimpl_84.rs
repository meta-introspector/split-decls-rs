// Generated macro for impl_84 (impl)
macro_rules! Depcrate_intimpl_84 {
() => {
// Module: crate::int
// Provides: {"impl_84"}
// Dependencies: {}
# [doc = " `P(Ul) - P(Ur)`: We resolve this with our `PrivateAdd`"] impl < Ul : Unsigned + NonZero , Ur : Unsigned + NonZero > Sub < PInt < Ur > > for PInt < Ul > where Ul : Cmp < Ur > + PrivateIntegerAdd < < Ul as Cmp < Ur > > :: Output , Ur > , { type Output = < Ul as PrivateIntegerAdd < < Ul as Cmp < Ur > > :: Output , Ur > > :: Output ; # [inline] fn sub (self , rhs : PInt < Ur >) -> Self :: Output { let lhs = self . n ; let rhs = rhs . n ; let lhs_cmp_rhs = lhs . compare :: < Internal > (& rhs) ; lhs . private_integer_add (lhs_cmp_rhs , rhs) } }
};
}
