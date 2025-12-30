// Generated macro for Nonce (struct)
macro_rules! Depcrate_nonceNonce {
() => {
// Module: crate::nonce
// Provides: {"Nonce"}
// Dependencies: {}
# [doc = " A \"nonce\" is a value that gets created exactly once."] # [doc = " We use it to mark the database storage so we can be sure we're seeing the same database."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct Nonce < T > (NonZeroU32 , PhantomData < T >) ;
};
}
