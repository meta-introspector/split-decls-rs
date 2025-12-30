// Generated macro for FieldWriterRaw (struct)
macro_rules! Depcrate_genericFieldWriterRaw {
() => {
// Module: crate::generic
// Provides: {"FieldWriterRaw"}
// Dependencies: {}
# [doc (hidden)] pub struct FieldWriterRaw < 'a , U , REG , N , FI , Safety , const WI : u8 , const O : u8 > where REG : Writable + RegisterSpec < Ux = U > , FI : Into < N > , { pub (crate) w : & 'a mut REG :: Writer , _field : marker :: PhantomData < (N , FI , Safety) > , }
};
}
