macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Drop for Sha256VarCore { fn drop (& mut self) { # [cfg (feature = "zeroize")] { use digest :: zeroize :: Zeroize ; self . state . zeroize () ; self . block_len . zeroize () ; } } }
    };
}

impl_10!();