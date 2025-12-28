macro_rules! deps {
    () => {
        ABI!();
    };
}

macro_rules! AbiKind {
    () => {
        deps!();
        # [doc = " Indicates the variant of the AArch64 ABI we are compiling for."] # [doc = " Used to accommodate Apple and Microsoft's deviations from the usual AAPCS ABI."] # [doc = ""] # [doc = " Corresponds to Clang's `AArch64ABIInfo::ABIKind`."] # [derive (Copy , Clone , PartialEq)] pub (crate) enum AbiKind { AAPCS , DarwinPCS , Win64 , }
    };
}

AbiKind!()