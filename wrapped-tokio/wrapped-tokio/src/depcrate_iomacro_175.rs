// Generated macro for macro_175 (macro)
macro_rules! Depcrate_iomacro_175 {
() => {
// Module: crate::io
// Provides: {"macro_175"}
// Dependencies: {}
cfg_io_util ! { mod split ; pub use split :: { split , ReadHalf , WriteHalf } ; mod join ; pub use join :: { join , Join } ; pub (crate) mod seek ; pub (crate) mod util ; pub use util :: { copy , copy_bidirectional , copy_bidirectional_with_sizes , copy_buf , duplex , empty , repeat , sink , simplex , AsyncBufReadExt , AsyncReadExt , AsyncSeekExt , AsyncWriteExt , BufReader , BufStream , BufWriter , Chain , DuplexStream , Empty , Lines , Repeat , Sink , Split , Take , SimplexStream , } ; }
};
}
