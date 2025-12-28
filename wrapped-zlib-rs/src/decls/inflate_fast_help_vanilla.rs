macro_rules! deps {
    () => {
        State!();
        CpuFeatures!();
    };
}

macro_rules! inflate_fast_help_vanilla {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = ""] # [doc = " `state.bit_reader` must have at least 15 bytes available to read, as"] # [doc = " indicated by `state.bit_reader.bytes_remaining() >= 15`"] unsafe fn inflate_fast_help_vanilla (state : & mut State , start : usize) { unsafe { inflate_fast_help_impl :: < { CpuFeatures :: NONE } > (state , start) } ; }
    };
}

inflate_fast_help_vanilla!()