// Generated macro for impl_34 (impl)
macro_rules! Depcrate_pointimpl_34 {
() => {
// Module: crate::point
// Provides: {"impl_34"}
// Dependencies: {}
impl < Size : ModulusSize > Coordinates < '_ , Size > { # [doc = " Get the tag octet needed to encode this set of [`Coordinates`]"] pub fn tag (& self) -> Tag { match self { Coordinates :: Compact { .. } => Tag :: Compact , Coordinates :: Compressed { y_is_odd , .. } => { if * y_is_odd { Tag :: CompressedOddY } else { Tag :: CompressedEvenY } } Coordinates :: Identity => Tag :: Identity , Coordinates :: Uncompressed { .. } => Tag :: Uncompressed , } } }
};
}
