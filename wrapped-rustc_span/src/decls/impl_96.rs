macro_rules! deps {
    () => {
        DefPathHash!();
        StableCrateId!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl DefPathHash { # [doc = " Returns the [StableCrateId] identifying the crate this [DefPathHash]"] # [doc = " originates from."] # [inline] pub fn stable_crate_id (& self) -> StableCrateId { StableCrateId (self . 0 . split () . 0) } # [doc = " Returns the crate-local part of the [DefPathHash]."] # [inline] pub fn local_hash (& self) -> Hash64 { self . 0 . split () . 1 } # [doc = " Builds a new [DefPathHash] with the given [StableCrateId] and"] # [doc = " `local_hash`, where `local_hash` must be unique within its crate."] # [inline] pub fn new (stable_crate_id : StableCrateId , local_hash : Hash64) -> DefPathHash { DefPathHash (Fingerprint :: new (stable_crate_id . 0 , local_hash)) } }
    };
}

impl_96!()