// Generated macro for impl_194 (impl)
macro_rules! Depcrate_tidimpl_194 {
() => {
// Module: crate::tid
// Provides: {"impl_194"}
// Dependencies: {}
impl < C : cfg :: Config > Pack < C > for Tid < C > { const LEN : usize = C :: MAX_SHARDS . trailing_zeros () as usize + 1 ; type Prev = page :: Addr < C > ; # [inline (always)] fn as_usize (& self) -> usize { self . id } # [inline (always)] fn from_usize (id : usize) -> Self { Self { id , _not_send : PhantomData , _cfg : PhantomData , } } }
};
}
