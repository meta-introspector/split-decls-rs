macro_rules! deps {
    () => {
        Capability!();
    };
}

macro_rules! CompatCapability {
    () => {
        deps!();
        # [doc = " Compatibility trait to keep existing code that uses the deprecated [`Capability`] type working."] # [doc = ""] # [doc = " This trait and its methods are sealed. It must not be used downstream."] pub trait CompatCapability : private :: Sealed + Copy { # [doc (hidden)] fn as_capability_set (self , _ : private :: Token) -> CapabilitySet ; }
    };
}

CompatCapability!()