macro_rules! deps {
    () => {
        State!();
        CpuFeatures!();
    };
}

macro_rules! inflate_fast_help_avx2 {
    () => {
        deps!();
        # [doc = " # Safety"] # [doc = ""] # [doc = " `state.bit_reader` must have at least 15 bytes available to read, as"] # [doc = " indicated by `state.bit_reader.bytes_remaining() >= 15`"] # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] # [target_feature (enable = "avx2")] # [target_feature (enable = "bmi2")] # [target_feature (enable = "bmi1")] unsafe fn inflate_fast_help_avx2 (state : & mut State , start : usize) { unsafe { inflate_fast_help_impl :: < { CpuFeatures :: AVX2 } > (state , start) } ; }
    };
}

inflate_fast_help_avx2!();