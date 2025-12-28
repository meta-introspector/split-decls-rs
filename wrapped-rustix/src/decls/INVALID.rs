macro_rules! INVALID {
    () => {
        const INVALID : * mut c_void = 1 as * mut c_void ;
    };
}

INVALID!();