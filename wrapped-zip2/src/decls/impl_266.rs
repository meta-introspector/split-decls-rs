macro_rules! deps {
    () => {
        StreamWriter!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < W : Write > StreamWriter < W > { # [doc = " Creates an instance wrapping the provided inner writer."] pub fn new (inner : W) -> StreamWriter < W > { Self { inner , bytes_written : 0 , } } # [doc = " Consumes this wrapper, returning the underlying writer."] pub fn into_inner (self) -> W { self . inner } }
    };
}

impl_266!();