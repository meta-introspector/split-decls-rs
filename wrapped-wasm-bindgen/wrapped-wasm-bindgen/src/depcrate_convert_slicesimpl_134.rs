// Generated macro for impl_134 (impl)
macro_rules! Depcrate_convert_slicesimpl_134 {
() => {
// Module: crate::convert::slices
// Provides: {"impl_134"}
// Dependencies: {}
impl WasmAbi for WasmMutSlice { # [doc = " `self.slice.ptr`"] type Prim1 = u32 ; # [doc = " `self.slice.len`"] type Prim2 = u32 ; # [doc = " `self.idx`"] type Prim3 = u32 ; type Prim4 = () ; # [inline] fn split (self) -> (u32 , u32 , u32 , ()) { (self . slice . ptr , self . slice . len , self . idx , ()) } # [inline] fn join (ptr : u32 , len : u32 , idx : u32 , _ : ()) -> Self { Self { slice : WasmSlice { ptr , len } , idx , } } }
};
}
