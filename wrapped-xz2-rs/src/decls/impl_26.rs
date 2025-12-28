macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: Data => "lzma data error" , Error :: Options => "invalid options" , Error :: Format => "stream/file format not recognized" , Error :: MemLimit => "memory limit reached" , Error :: Mem => "can't allocate memory" , Error :: Program => "liblzma internal error" , Error :: NoCheck => "no integrity check was available" , Error :: UnsupportedCheck => "liblzma not built with check support" , } . fmt (f) } }
    };
}

impl_26!();