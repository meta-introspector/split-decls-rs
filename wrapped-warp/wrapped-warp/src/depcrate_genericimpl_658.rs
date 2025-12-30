// Generated macro for impl_658 (impl)
macro_rules! Depcrate_genericimpl_658 {
() => {
// Module: crate::generic
// Provides: {"impl_658"}
// Dependencies: {}
impl < H , T : HList , U : HList > Combine < U > for Product < H , T > where T : Combine < U > , Product < H , < T as Combine < U > > :: Output > : HList , { type Output = Product < H , < T as Combine < U > > :: Output > ; # [inline] fn combine (self , other : U) -> Self :: Output { Product (self . 0 , self . 1 . combine (other)) } }
};
}
