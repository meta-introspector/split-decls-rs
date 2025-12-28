macro_rules! deps {
    () => {
        TypingMode!();
        Canonical!();
        Interner!();
    };
}

macro_rules! CanonicalQueryInput {
    () => {
        deps!();
        # [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , V)] # [derive_where (Copy ; I : Interner , V : Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct CanonicalQueryInput < I : Interner , V > { pub canonical : Canonical < I , V > , pub typing_mode : TypingMode < I > , }
    };
}

CanonicalQueryInput!()