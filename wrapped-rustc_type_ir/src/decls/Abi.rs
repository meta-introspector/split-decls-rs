macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! Abi {
    () => {
        deps!();
        pub trait Abi < I : Interner < Abi = Self > > : Copy + Debug + Hash + Eq { fn rust () -> Self ; # [doc = " Whether this ABI is `extern \"Rust\"`."] fn is_rust (self) -> bool ; }
    };
}

Abi!();