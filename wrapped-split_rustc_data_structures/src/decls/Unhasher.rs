macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! Unhasher {
    () => {
        deps!();
        # [doc = " This no-op hasher expects only a single `write_u64` call. It's intended for"] # [doc = " map keys that already have hash-like quality, like `Fingerprint`."] # [derive (Default)] pub struct Unhasher { value : u64 , }
    };
}

Unhasher!();