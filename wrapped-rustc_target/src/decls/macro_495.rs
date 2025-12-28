macro_rules! deps {
    () => {
        ABI!();
    };
}

macro_rules! macro_495 {
    () => {
        deps!();
        crate :: target_spec_enum ! { # [doc = " The Rustc-specific variant of the ABI used for this target."] pub enum RustcAbi { # [doc = " On x86-32 only: make use of SSE and SSE2 for ABI purposes."] X86Sse2 = "x86-sse2" , # [doc = " On x86-32/64 only: do not use any FPU or SIMD registers for the ABI."] X86Softfloat = "x86-softfloat" , } parse_error_type = "rustc abi" ; }
    };
}

macro_495!();