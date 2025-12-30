// Generated macro for new_empty_allocation (function)
macro_rules! Depcrate_allocnew_empty_allocation {
() => {
// Module: crate::alloc
// Provides: {"new_empty_allocation"}
// Dependencies: {}
# [doc = " Creates new empty `Allocation` from given `Align`."] fn new_empty_allocation (align : Align) -> Allocation { Allocation { bytes : Vec :: new () , provenance : ProvenanceMap { ptrs : Vec :: new () } , align : align . bytes () , mutability : Mutability :: Not , } }
};
}
