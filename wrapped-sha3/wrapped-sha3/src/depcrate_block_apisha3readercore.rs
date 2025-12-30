// Generated macro for Sha3ReaderCore (struct)
macro_rules! Depcrate_block_apiSha3ReaderCore {
() => {
// Module: crate::block_api
// Provides: {"Sha3ReaderCore"}
// Dependencies: {}
# [doc = " Core Sha3 XOF reader."] # [derive (Clone)] pub struct Sha3ReaderCore < Rate , const ROUNDS : usize = DEFAULT_ROUND_COUNT > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , { state : [u64 ; PLEN] , _pd : PhantomData < Rate > , }
};
}
