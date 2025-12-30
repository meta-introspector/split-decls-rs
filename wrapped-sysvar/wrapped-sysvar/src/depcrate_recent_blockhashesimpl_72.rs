// Generated macro for impl_72 (impl)
macro_rules! Depcrate_recent_blockhashesimpl_72 {
() => {
// Module: crate::recent_blockhashes
// Provides: {"impl_72"}
// Dependencies: {}
impl Entry { pub fn new (blockhash : & Hash , lamports_per_signature : u64) -> Self { Self { blockhash : * blockhash , fee_calculator : FeeCalculator :: new (lamports_per_signature) , } } }
};
}
