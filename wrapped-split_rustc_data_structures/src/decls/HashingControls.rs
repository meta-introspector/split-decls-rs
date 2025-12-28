macro_rules! deps {
    () => {
        HashStable!();
        Fingerprint!();
    };
}

macro_rules! HashingControls {
    () => {
        deps!();
        # [doc = " Controls what data we do or do not hash."] # [doc = " Whenever a `HashStable` implementation caches its"] # [doc = " result, it needs to include `HashingControls` as part"] # [doc = " of the key, to ensure that it does not produce an incorrect"] # [doc = " result (for example, using a `Fingerprint` produced while"] # [doc = " hashing `Span`s when a `Fingerprint` without `Span`s is"] # [doc = " being requested)"] # [derive (Clone , Hash , Eq , PartialEq , Debug)] pub struct HashingControls { pub hash_spans : bool , }
    };
}

HashingControls!();