// Generated macro for impl_70 (impl)
macro_rules! Depcrate_convert_implsimpl_70 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_70"}
// Dependencies: {}
impl WasmAbi for u128 { type Prim1 = u64 ; type Prim2 = u64 ; type Prim3 = () ; type Prim4 = () ; # [inline] fn split (self) -> (u64 , u64 , () , ()) { let low = self as u64 ; let high = (self >> 64) as u64 ; (low , high , () , ()) } # [inline] fn join (low : u64 , high : u64 , _ : () , _ : ()) -> Self { (high as u128) << 64 | low as u128 } }
};
}
