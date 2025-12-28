macro_rules! macro_140 {
    () => {
        cfg_io_util ! { mod split ; pub use split :: { split , ReadHalf , WriteHalf } ; mod join ; pub use join :: { join , Join } ; pub (crate) mod seek ; pub (crate) mod util ; pub use util :: { copy , copy_bidirectional , copy_bidirectional_with_sizes , copy_buf , duplex , empty , repeat , sink , simplex , AsyncBufReadExt , AsyncReadExt , AsyncSeekExt , AsyncWriteExt , BufReader , BufStream , BufWriter , Chain , DuplexStream , Empty , Lines , Repeat , Sink , Split , Take , SimplexStream , } ; }
    };
}

macro_140!()