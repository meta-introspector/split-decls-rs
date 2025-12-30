// Generated macro for Fees (struct)
macro_rules! Depcrate_feesFees {
() => {
// Module: crate::fees
// Provides: {"Fees"}
// Dependencies: {}
# [doc = " Transaction fees."] # [deprecated (since = "1.9.0" , note = "Please do not use, will no longer be available in the future")] # [repr (C)] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , CloneZeroed , Default , PartialEq , Eq)] pub struct Fees { pub fee_calculator : FeeCalculator , }
};
}
