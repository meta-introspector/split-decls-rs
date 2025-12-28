macro_rules! RawGid {
    () => {
        # [doc = " A group identifier as a raw integer."] pub type RawGid = ffi :: c_uint ;
    };
}

RawGid!();