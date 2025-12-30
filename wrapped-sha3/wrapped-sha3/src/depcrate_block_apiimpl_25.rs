// Generated macro for impl_25 (impl)
macro_rules! Depcrate_block_apiimpl_25 {
() => {
// Module: crate::block_api
// Provides: {"impl_25"}
// Dependencies: {}
impl < Rate , const ROUNDS : usize > XofReaderCore for Sha3ReaderCore < Rate , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , { # [inline] fn read_block (& mut self) -> Block < Self > { let mut block = Block :: < Self > :: default () ; for (src , dst) in self . state . iter () . zip (block . chunks_mut (8)) { dst . copy_from_slice (& src . to_le_bytes () [.. dst . len ()]) ; } keccak :: p1600 (& mut self . state , ROUNDS) ; block } }
};
}
