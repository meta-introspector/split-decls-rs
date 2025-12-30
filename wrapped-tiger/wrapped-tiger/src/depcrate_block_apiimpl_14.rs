// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl < const V2 : bool > UpdateCore for TigerCore < V2 > { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len += blocks . len () as u64 ; for block in blocks { compress (& mut self . state , block . as_ref ()) ; } } }
};
}
