macro_rules! ArmHasGuard {
    () => {
        # [doc = " `ArmHasGuard` is a wrapper around a boolean flag. It indicates whether"] # [doc = " a match arm has a guard expression attached to it."] # [derive (Copy , Clone , Debug)] pub (crate) struct ArmHasGuard (pub (crate) bool) ;
    };
}

ArmHasGuard!();