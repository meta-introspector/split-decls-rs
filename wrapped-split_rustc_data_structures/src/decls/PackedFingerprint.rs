macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! PackedFingerprint {
    () => {
        deps!();
        # [cfg_attr (any (target_arch = "x86" , target_arch = "x86_64") , repr (packed))] # [derive (Eq , PartialEq , Ord , PartialOrd , Debug , Clone , Copy , Hash)] pub struct PackedFingerprint (Fingerprint) ;
    };
}

PackedFingerprint!();