macro_rules! deps {
    () => {
        Target!();
        ABI!();
        OsKind!();
        Arch!();
    };
}

macro_rules! AbiMap {
    () => {
        deps!();
        # [doc = " Mapping for ExternAbi to CanonAbi according to a Target"] # [doc = ""] # [doc = " A maybe-transitional structure circa 2025 for hosting future experiments in"] # [doc = " encapsulating arch-specific ABI lowering details to make them more testable."] # [derive (Clone , Debug)] pub struct AbiMap { arch : Arch , os : OsKind , }
    };
}

AbiMap!();