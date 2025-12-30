// Generated macro for impl_744 (impl)
macro_rules! Depcrate_nonceimpl_744 {
() => {
// Module: crate::nonce
// Provides: {"impl_744"}
// Dependencies: {}
impl < T > NonceGenerator < T > { pub (crate) const fn new () -> Self { Self { value : AtomicU32 :: new (1) , phantom : PhantomData , } } pub (crate) fn nonce (& self) -> Nonce < T > { let value = self . value . fetch_add (1 , Ordering :: Relaxed) ; assert ! (value != 0 , "nonce rolled over") ; Nonce (NonZeroU32 :: new (value) . unwrap () , self . phantom) } }
};
}
