macro_rules! deps {
    () => {
        StableCrateId!();
        ExpnHash!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ExpnHash { # [doc = " Returns the [StableCrateId] identifying the crate this [ExpnHash]"] # [doc = " originates from."] # [inline] pub fn stable_crate_id (self) -> StableCrateId { StableCrateId (self . 0 . split () . 0) } # [doc = " Returns the crate-local part of the [ExpnHash]."] # [doc = ""] # [doc = " Used for assertions."] # [inline] pub fn local_hash (self) -> Hash64 { self . 0 . split () . 1 } # [inline] pub fn is_root (self) -> bool { self . 0 == Fingerprint :: ZERO } # [doc = " Builds a new [ExpnHash] with the given [StableCrateId] and"] # [doc = " `local_hash`, where `local_hash` must be unique within its crate."] fn new (stable_crate_id : StableCrateId , local_hash : Hash64) -> ExpnHash { ExpnHash (Fingerprint :: new (stable_crate_id . 0 , local_hash)) } }
    };
}

impl_50!();