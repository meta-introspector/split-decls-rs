// Generated macro for VisitMap (enum)
macro_rules! Depcrate_deVisitMap {
() => {
// Module: crate::de
// Provides: {"VisitMap"}
// Dependencies: {}
# [doc = " Integrate [`Datetime`][crate::Datetime] into an untagged deserialize"] # [cfg (feature = "alloc")] pub enum VisitMap < 'de > { # [doc = " The map was deserialized as a [Datetime][crate::Datetime] value"] Datetime (crate :: Datetime) , # [doc = " The map is of an unknown format and needs further deserialization"] Key (alloc :: borrow :: Cow < 'de , str >) , }
};
}
