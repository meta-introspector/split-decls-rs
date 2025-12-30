// Generated macro for VecGrowScanItem (struct)
macro_rules! DepcrateVecGrowScanItem {
() => {
// Module: crate
// Provides: {"VecGrowScanItem"}
// Dependencies: {}
# [doc = " Reference wrapper that enables item insertion and removal for [`VecGrowScan`]."] # [repr (transparent)] pub struct VecGrowScanItem < 's , 'a , T : 'a > { scan : & 's mut VecGrowScan < 'a , T > , }
};
}
