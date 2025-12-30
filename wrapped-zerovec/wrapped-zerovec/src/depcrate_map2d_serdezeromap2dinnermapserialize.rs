// Generated macro for ZeroMap2dInnerMapSerialize (struct)
macro_rules! Depcrate_map2d_serdeZeroMap2dInnerMapSerialize {
() => {
// Module: crate::map2d::serde
// Provides: {"ZeroMap2dInnerMapSerialize"}
// Dependencies: {}
# [doc = " Helper struct for human-serializing the inner map of a ZeroMap2d"] # [cfg (feature = "serde")] struct ZeroMap2dInnerMapSerialize < 'a , 'l , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized + Ord , K1 : ZeroMapKV < 'a > + ? Sized + Ord , V : ZeroMapKV < 'a > + ? Sized , { pub cursor : ZeroMap2dCursor < 'l , 'a , K0 , K1 , V > , }
};
}
