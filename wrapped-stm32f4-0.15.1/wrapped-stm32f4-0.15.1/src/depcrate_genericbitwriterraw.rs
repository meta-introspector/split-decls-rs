// Generated macro for BitWriterRaw (struct)
macro_rules! Depcrate_genericBitWriterRaw {
() => {
// Module: crate::generic
// Provides: {"BitWriterRaw"}
// Dependencies: {}
# [doc (hidden)] pub struct BitWriterRaw < 'a , U , REG , FI , M , const O : u8 > where REG : Writable + RegisterSpec < Ux = U > , FI : Into < bool > , { pub (crate) w : & 'a mut REG :: Writer , _field : marker :: PhantomData < (FI , M) > , }
};
}
