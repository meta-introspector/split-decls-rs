// Generated macro for impl_32 (impl)
macro_rules! Depcrate_genericimpl_32 {
() => {
// Module: crate::generic
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a , U , REG , N , FI , Safety , const WI : u8 , const O : u8 > FieldWriterRaw < 'a , U , REG , N , FI , Safety , WI , O > where REG : Writable + RegisterSpec < Ux = U > , FI : Into < N > , { # [doc = " Creates a new instance of the writer"] # [allow (unused)] # [inline (always)] pub (crate) fn new (w : & 'a mut REG :: Writer) -> Self { Self { w , _field : marker :: PhantomData , } } }
};
}
