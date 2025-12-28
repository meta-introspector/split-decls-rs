macro_rules! deps {
    () => {
        X86Abi!();
    };
}

macro_rules! HasX86AbiOpt {
    () => {
        deps!();
        pub trait HasX86AbiOpt { fn x86_abi_opt (& self) -> X86Abi ; }
    };
}

HasX86AbiOpt!();