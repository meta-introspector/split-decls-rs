macro_rules! deps {
    () => {
        Stability!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl Stability { # [doc = " Returns whether the feature can be used in `cfg(target_feature)` ever."] # [doc = " (It might still be nightly-only even if this returns `true`, so make sure to also check"] # [doc = " `requires_nightly`.)"] pub fn in_cfg (& self) -> bool { matches ! (self , Stability :: Stable | Stability :: Unstable { .. }) } # [doc = " Returns the nightly feature that is required to toggle this target feature via"] # [doc = " `#[target_feature]`/`-Ctarget-feature` or to test it via `cfg(target_feature)`."] # [doc = " (For `cfg` we only care whether the feature is nightly or not, we don't require"] # [doc = " the feature gate to actually be enabled when using a nightly compiler.)"] # [doc = ""] # [doc = " Before calling this, ensure the feature is even permitted for this use:"] # [doc = " - for `#[target_feature]`/`-Ctarget-feature`, check `allow_toggle()`"] # [doc = " - for `cfg(target_feature)`, check `in_cfg`"] pub fn requires_nightly (& self) -> Option < Symbol > { match * self { Stability :: Unstable (nightly_feature) => Some (nightly_feature) , Stability :: Stable { .. } => None , Stability :: Forbidden { .. } => panic ! ("forbidden features should not reach this far") , } } # [doc = " Returns whether the feature may be toggled via `#[target_feature]` or `-Ctarget-feature`."] # [doc = " (It might still be nightly-only even if this returns `true`, so make sure to also check"] # [doc = " `requires_nightly`.)"] pub fn toggle_allowed (& self) -> Result < () , & 'static str > { match self { Stability :: Unstable (_) | Stability :: Stable { .. } => Ok (()) , Stability :: Forbidden { reason } => Err (reason) , } } }
    };
}

impl_557!();