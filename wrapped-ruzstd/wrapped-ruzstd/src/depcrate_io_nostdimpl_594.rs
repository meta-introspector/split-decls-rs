// Generated macro for impl_594 (impl)
macro_rules! Depcrate_io_nostdimpl_594 {
() => {
// Module: crate::io_nostd
// Provides: {"impl_594"}
// Dependencies: {}
impl Write for & mut [u8] { # [inline] fn write (& mut self , data : & [u8]) -> Result < usize , Error > { let amt = core :: cmp :: min (data . len () , self . len ()) ; let (a , b) = core :: mem :: take (self) . split_at_mut (amt) ; a . copy_from_slice (& data [.. amt]) ; * self = b ; Ok (amt) } fn flush (& mut self) -> Result < () , Error > { Ok (()) } }
};
}
