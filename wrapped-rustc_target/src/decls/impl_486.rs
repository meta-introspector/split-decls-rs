macro_rules! deps {
    () => {
        SmallDataThresholdSupport!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl FromStr for SmallDataThresholdSupport { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if s == "none" { Ok (Self :: None) } else if s == "default-for-arch" { Ok (Self :: DefaultForArch) } else if let Some (flag) = s . strip_prefix ("llvm-module-flag=") { Ok (Self :: LlvmModuleFlag (flag . to_string () . into ())) } else if let Some (arg) = s . strip_prefix ("llvm-arg=") { Ok (Self :: LlvmArg (arg . to_string () . into ())) } else { Err (format ! ("'{s}' is not a valid value for small-data-threshold-support.")) } } }
    };
}

impl_486!();