macro_rules! RawUid {
    () => {
        # [doc = " A user identifier as a raw integer."] pub type RawUid = ffi :: c_uint ;
    };
}

RawUid!();