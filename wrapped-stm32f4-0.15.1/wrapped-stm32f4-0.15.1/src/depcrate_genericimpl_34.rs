// Generated macro for impl_34 (impl)
macro_rules! Depcrate_genericimpl_34 {
() => {
// Module: crate::generic
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , U , REG , FI , M , const O : u8 > BitWriterRaw < 'a , U , REG , FI , M , O > where REG : Writable + RegisterSpec < Ux = U > , FI : Into < bool > , { # [doc = " Creates a new instance of the writer"] # [allow (unused)] # [inline (always)] pub (crate) fn new (w : & 'a mut REG :: Writer) -> Self { Self { w , _field : marker :: PhantomData , } } }
};
}
