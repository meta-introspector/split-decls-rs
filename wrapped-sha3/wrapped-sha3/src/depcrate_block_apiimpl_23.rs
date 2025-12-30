// Generated macro for impl_23 (impl)
macro_rules! Depcrate_block_apiimpl_23 {
() => {
// Module: crate::block_api
// Provides: {"impl_23"}
// Dependencies: {}
impl < Rate , const ROUNDS : usize > Sha3ReaderCore < Rate , ROUNDS > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , { pub (crate) fn new (state : & [u64 ; PLEN]) -> Self { Self { state : * state , _pd : PhantomData , } } }
};
}
