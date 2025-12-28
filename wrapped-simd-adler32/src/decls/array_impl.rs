macro_rules! deps {
    () => {
        Adler32!();
        Adler32Hash!();
    };
}

macro_rules! array_impl {
    () => {
        deps!();
        macro_rules ! array_impl { ($ s : expr , $ ($ size : expr) ,+) => { array_impl ! ($ s) ; $ (array_impl ! { $ size }) * } ; ($ size : expr) => { # [cfg (not (feature = "const-generics"))] impl Adler32Hash for [u8 ; $ size] { fn hash (& self) -> u32 { let mut hash = Adler32 :: new () ; hash . write (self) ; hash . finish () } } } ; }
    };
}

array_impl!()