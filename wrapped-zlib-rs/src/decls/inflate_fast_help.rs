macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! inflate_fast_help {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = ""] # [doc = " `state.bit_reader` must have at least 15 bytes available to read, as"] # [doc = " indicated by `state.bit_reader.bytes_remaining() >= 15`"] unsafe fn inflate_fast_help (state : & mut State , start : usize) { # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] if crate :: cpu_features :: is_enabled_avx2_and_bmi2 () { return unsafe { inflate_fast_help_avx2 (state , start) } ; } unsafe { inflate_fast_help_vanilla (state , start) } ; }
    };
}

inflate_fast_help!()