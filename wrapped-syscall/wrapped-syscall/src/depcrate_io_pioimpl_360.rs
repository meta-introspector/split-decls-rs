// Generated macro for impl_360 (impl)
macro_rules! Depcrate_io_pioimpl_360 {
() => {
// Module: crate::io::pio
// Provides: {"impl_360"}
// Dependencies: {}
impl < T > Pio < T > { # [doc = " Create a PIO from a given port"] pub const fn new (port : u16) -> Self { Pio :: < T > { port , value : PhantomData , } } }
};
}
