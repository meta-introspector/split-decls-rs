// Generated macro for impl_131 (impl)
macro_rules! Depcrate_convert_slicesimpl_131 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_131"}
// Dependencies: {}
impl WasmAbi for WasmSlice { # [doc = " `self.ptr`"] type Prim1 = u32 ; # [doc = " `self.len`"] type Prim2 = u32 ; type Prim3 = () ; type Prim4 = () ; # [inline] fn split (self) -> (u32 , u32 , () , ()) { (self . ptr , self . len , () , ()) } # [inline] fn join (ptr : u32 , len : u32 , _ : () , _ : ()) -> Self { Self { ptr , len } } }
};
}
