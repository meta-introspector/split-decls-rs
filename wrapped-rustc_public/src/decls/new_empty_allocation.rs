macro_rules! deps {
    () => {
        Align!();
        ProvenanceMap!();
        Allocation!();
    };
}

macro_rules! new_empty_allocation {
    () => {
        deps!();
        # [doc = " Creates new empty `Allocation` from given `Align`."] fn new_empty_allocation (align : Align) -> Allocation { Allocation { bytes : Vec :: new () , provenance : ProvenanceMap { ptrs : Vec :: new () } , align : align . bytes () , mutability : Mutability :: Not , } }
    };
}

new_empty_allocation!()