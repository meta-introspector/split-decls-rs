// Generated macro for impl_68 (impl)
macro_rules! Depcrate_convert_implsimpl_68 {
() => {
// Module: crate::convert::impls
// Provides: {"impl_68"}
// Dependencies: {}
impl < T : WasmPrimitive > WasmAbi for T { type Prim1 = Self ; type Prim2 = () ; type Prim3 = () ; type Prim4 = () ; # [inline] fn split (self) -> (Self , () , () , ()) { (self , () , () , ()) } # [inline] fn join (prim : Self , _ : () , _ : () , _ : ()) -> Self { prim } }
};
}
