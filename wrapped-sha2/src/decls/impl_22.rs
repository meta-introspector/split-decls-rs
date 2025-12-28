macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Drop for Sha512VarCore { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . state . zeroize () ; self . block_len . zeroize () ; } } }
    };
}

impl_22!();