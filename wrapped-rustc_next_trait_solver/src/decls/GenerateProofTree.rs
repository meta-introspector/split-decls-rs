macro_rules! GenerateProofTree {
    () => {
        # [derive (PartialEq , Eq , Debug , Hash , Clone , Copy)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum GenerateProofTree { Yes , No , }
    };
}

GenerateProofTree!();