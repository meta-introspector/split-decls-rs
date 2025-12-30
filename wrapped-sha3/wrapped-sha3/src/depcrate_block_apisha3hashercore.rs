// Generated macro for Sha3HasherCore (struct)
macro_rules! Depcrate_block_apiSha3HasherCore {
() => {
// Module: crate::block_api
// Provides: {"Sha3HasherCore"}
// Dependencies: {}
# [doc = " Core Sha3 fixed output hasher state."] # [derive (Clone)] pub struct Sha3HasherCore < Rate , OutputSize , const PAD : u8 , const ROUNDS : usize = DEFAULT_ROUND_COUNT , > where Rate : BlockSizes + IsLessOrEqual < U200 , Output = True > , OutputSize : ArraySize + IsLessOrEqual < U200 , Output = True > , { state : [u64 ; PLEN] , _pd : PhantomData < (Rate , OutputSize) > , }
};
}
