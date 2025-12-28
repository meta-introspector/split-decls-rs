macro_rules! deps {
    () => {
        AbiMapping!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl AbiMapping { # [doc = " optionally get a [CanonAbi], even if Deprecated"] pub fn into_option (self) -> Option < CanonAbi > { match self { Self :: Direct (abi) | Self :: Deprecated (abi) => Some (abi) , Self :: Invalid => None , } } # [doc = " get a [CanonAbi] even if Deprecated, panicking if Invalid"] # [track_caller] pub fn unwrap (self) -> CanonAbi { self . into_option () . unwrap () } pub fn is_mapped (self) -> bool { self . into_option () . is_some () } }
    };
}

impl_323!()