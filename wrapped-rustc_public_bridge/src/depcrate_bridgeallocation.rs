// Generated macro for Allocation (trait)
macro_rules! Depcrate_bridgeAllocation {
() => {
// Module: crate::bridge
// Provides: {"Allocation"}
// Dependencies: {}
pub trait Allocation < B : Bridge > { fn new < 'tcx > (bytes : Vec < Option < u8 > > , ptrs : Vec < (usize , rustc_middle :: mir :: interpret :: AllocId) > , align : u64 , mutability : rustc_middle :: mir :: Mutability , tables : & mut Tables < 'tcx , B > , cx : & CompilerCtxt < 'tcx , B > ,) -> Self ; }
};
}
