macro_rules! deps {
    () => {
        ToJson!();
        SmallDataThresholdSupport!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl ToJson for SmallDataThresholdSupport { fn to_json (& self) -> Value { match self { Self :: None => "none" . to_json () , Self :: DefaultForArch => "default-for-arch" . to_json () , Self :: LlvmModuleFlag (flag) => format ! ("llvm-module-flag={flag}") . to_json () , Self :: LlvmArg (arg) => format ! ("llvm-arg={arg}") . to_json () , } } }
    };
}

impl_489!();