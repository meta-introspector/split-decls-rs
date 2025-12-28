macro_rules! deps {
    () => {
        ABI!();
    };
}

macro_rules! AbiMapping {
    () => {
        deps!();
        # [doc = " result from trying to map an ABI"] # [derive (Copy , Clone , Debug)] pub enum AbiMapping { # [doc = " this ABI is exactly mapped for this platform"] Direct (CanonAbi) , # [doc = " we don't yet warn on this, but we will"] Deprecated (CanonAbi) , # [doc = " ABI we do not map for this platform: it must not reach codegen"] Invalid , }
    };
}

AbiMapping!();