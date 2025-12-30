// Generated macro for tests (module)
macro_rules! Depcrate_feestests {
() => {
// Module: crate::fees
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_clone () { let fees = Fees { fee_calculator : FeeCalculator { lamports_per_signature : 1 , } , } ; let cloned_fees = fees . clone () ; assert_eq ! (cloned_fees , fees) ; } }
};
}
