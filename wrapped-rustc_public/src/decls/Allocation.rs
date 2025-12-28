macro_rules! deps {
    () => {
        ProvenanceMap!();
        Align!();
        Bytes!();
    };
}

macro_rules! Allocation {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Allocation { pub bytes : Bytes , pub provenance : ProvenanceMap , pub align : Align , pub mutability : Mutability , }
    };
}

Allocation!()