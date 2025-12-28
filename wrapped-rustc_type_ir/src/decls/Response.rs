macro_rules! deps {
    () => {
        Interner!();
        Certainty!();
        CanonicalVarValues!();
    };
}

macro_rules! Response {
    () => {
        deps!();
        # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct Response < I : Interner > { pub certainty : Certainty , pub var_values : CanonicalVarValues < I > , # [doc = " Additional constraints returned by this query."] pub external_constraints : I :: ExternalConstraints , }
    };
}

Response!();