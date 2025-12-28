macro_rules! deps {
    () => {
        Ppmd!();
        GenericZipWriter!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < W : Write + Seek > Debug for GenericZipWriter < W > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match self { Closed => f . write_str ("Closed") , Storer (w) => f . write_fmt (format_args ! ("Storer({w:?})")) , # [cfg (feature = "deflate-flate2")] GenericZipWriter :: Deflater (w) => { f . write_fmt (format_args ! ("Deflater({:?})" , w . get_ref ())) } # [cfg (feature = "deflate-zopfli")] GenericZipWriter :: ZopfliDeflater (_) => f . write_str ("ZopfliDeflater") , # [cfg (feature = "deflate-zopfli")] GenericZipWriter :: BufferedZopfliDeflater (_) => f . write_str ("BufferedZopfliDeflater") , # [cfg (feature = "bzip2")] GenericZipWriter :: Bzip2 (w) => f . write_fmt (format_args ! ("Bzip2({:?})" , w . get_ref ())) , # [cfg (feature = "zstd")] GenericZipWriter :: Zstd (w) => f . write_fmt (format_args ! ("Zstd({:?})" , w . get_ref ())) , # [cfg (feature = "xz")] GenericZipWriter :: Xz (w) => f . write_fmt (format_args ! ("Xz({:?})" , w . inner ())) , # [cfg (feature = "ppmd")] GenericZipWriter :: Ppmd (_) => f . write_fmt (format_args ! ("Ppmd8Encoder")) , } } }
    };
}

impl_233!();