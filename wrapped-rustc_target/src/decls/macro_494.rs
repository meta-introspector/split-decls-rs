macro_rules! deps {
    () => {
        ABI!();
    };
}

macro_rules! macro_494 {
    () => {
        deps!();
        crate :: target_spec_enum ! { # [doc = " The float ABI setting to be configured in the LLVM target machine."] pub enum FloatAbi { Soft = "soft" , Hard = "hard" , } parse_error_type = "float abi" ; }
    };
}

macro_494!();