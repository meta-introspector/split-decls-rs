macro_rules! deps {
    () => {
        Check!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Check { # [doc = " Test if this check is supported in this build of liblzma."] pub fn is_supported (& self) -> bool { let ret = unsafe { lzma_sys :: lzma_check_is_supported (* self as lzma_sys :: lzma_check) } ; ret != 0 } }
    };
}

impl_19!();