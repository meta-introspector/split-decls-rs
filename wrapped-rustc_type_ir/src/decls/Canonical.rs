macro_rules! deps {
    () => {
        Interner!();
    };
}

macro_rules! Canonical {
    () => {
        deps!();
        # [doc = " A \"canonicalized\" type `V` is one where all free inference"] # [doc = " variables have been rewritten to \"canonical vars\". These are"] # [doc = " numbered starting from 0 in order of first appearance."] # [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , V)] # [derive_where (Copy ; I : Interner , V : Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct Canonical < I : Interner , V > { pub value : V , pub max_universe : UniverseIndex , pub variables : I :: CanonicalVarKinds , }
    };
}

Canonical!()