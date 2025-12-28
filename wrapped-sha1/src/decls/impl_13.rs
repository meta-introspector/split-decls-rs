macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Drop for Sha1Core { fn drop (& mut self) { # [cfg (feature = "zeroize")] { self . h . zeroize () ; self . block_len . zeroize () ; } } }
    };
}

impl_13!()